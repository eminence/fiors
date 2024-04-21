use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use fiors::{get_material_db, types::Planet, FIOClient};
use once_cell::sync::OnceCell;
use ratatui::{prelude::*, widgets::*};
use widgets::{SharedWidgetState, WidgetEnum};

static CLIENT: OnceCell<FIOClient> = OnceCell::new();

mod widgets;

fn get_client() -> &'static FIOClient {
    let api_key = std::env::args()
        .nth(1)
        .unwrap_or("9dd5160d-acc8-493d-b222-d5f96273f677".into());
    CLIENT.get_or_init(|| {
        let mut client = FIOClient::new_with_key(api_key);
        client.local_cache_dir = Some(".fio_cache".into());
        client
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let client = get_client();

    println!("Logging in...");
    let app = App::new().await?;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let ret = run_mainloop(terminal, app).await;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;

    if let Err(err) = ret {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn get_events() -> io::Result<Option<Event>> {
    if event::poll(Duration::from_millis(500))? {
        return Ok(Some(event::read()?));
    }
    Ok(None)

    // if event::poll(std::time::Duration::from_millis(500))? {
    //     if let Event::Key(key) = event::read()? {
    //         if key.kind == event::KeyEventKind::Press {
    //             return Ok(Some(key.code));
    //         }
    //     }
    // }
    // Ok(None)
}

#[derive(Debug, Copy, Clone)]
enum SidebarMode {
    Production,
    Buildings,
    Inventory,
}

struct App {
    // client: &'static FIOClient,
    // username: String,
    current_tab: usize,
    planets: Vec<Planet>,
    current_widget: WidgetEnum,
    lm_widget: widgets::LocalMarketWidget,
    production_widgets: widgets::ProductionWidget,
    building_widget: widgets::BuildingsWidget,
    mode: SidebarMode,
    sidebar_state: TableState,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum NeedRefresh {
    APIRefresh,
    Redraw,
    No,
}

impl NeedRefresh {
    fn update(self, new: Self) -> Self {
        match self {
            Self::No => new,
            Self::APIRefresh => Self::APIRefresh,
            Self::Redraw => {
                if new == Self::APIRefresh {
                    Self::APIRefresh
                } else {
                    Self::Redraw
                }
            }
        }
    }
}

pub fn get_style_for_material(ticker: &str) -> Style {
    let material = get_material_db().get(ticker).unwrap();
    let bg = material.category.get_bg_color();
    let fg = material.category.get_fg_color();
    Style::default()
        .fg(Color::Rgb(fg.0, fg.1, fg.2))
        .bg(Color::Rgb(bg.0, bg.1, bg.2))
}

fn get_style_for_days(days: f32) -> Style {
    let idx = (days / 3.0).floor().min(5.0) as u8;
    if days < 21.0 {
        Style::default().fg(Color::Indexed(160 + (idx * 6)))
    } else {
        Style::default()
    }
}

/// Format an amount so it fits nicely into a 4-character cell
fn format_amount(amt: f32) -> String {
    if amt.abs() < 10.0 {
        format!("{:.2}", amt)
    } else if amt.abs() < 100.0 {
        format!("{:.1}", amt)
    } else if amt.abs() < 10000.0 {
        format!("{:.0}", amt)
    } else {
        format!("{:.0}k", amt / 1000.0)
    }
}

/// Format a price so it fits nicely into a 6-character cell
fn format_price(price: f32) -> String {
    if price.abs() < 10.0 {
        format!("{:.2}", price) // 9.12
    } else if price.abs() < 100.0 {
        format!("{:.1}", price) // 99.1
    } else if price.abs() < 1000.0 {
        format!("{:.0}", price) // 991
    } else if price.abs() < 10000.0 {
        format!("{:.0}", price) // 9912
    } else {
        format!("{:.0}k", price / 1000.0)
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

impl App {
    async fn new() -> anyhow::Result<Self> {
        let client = get_client();
        let username = client.is_auth().await?;

        let planets = client.get_storage_planets_for_user(&username).await?;

        Ok(Self {
            lm_widget: widgets::LocalMarketWidget::new(client, &username, &planets[0].id),
            production_widgets: widgets::ProductionWidget::new(client, &username, &planets[0].id),
            building_widget: widgets::BuildingsWidget::new(client, &username, &planets[0].id),
            // client,
            // username,
            current_tab: 0,
            current_widget: WidgetEnum::Production,
            planets,
            mode: SidebarMode::Production,
            sidebar_state: TableState::default(),
        })
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let tab_names = self.planets.iter().map(|p| p.name.clone());
        let selected = self.current_tab;
        let tabs = Tabs::<'static>::new(tab_names)
            .block(Block::default().title("Bases").borders(Borders::ALL))
            .select(selected);

        frame.render_widget(tabs, area);
    }
    fn render_sidebar(&mut self, frame: &mut Frame, area: Rect) {
        let table = Table::new(
            [
                Row::new(vec!["\nPRD\n"]).height(3),
                Row::new(vec!["\nBLD\n"]).height(3),
                Row::new(vec!["\nINV\n"]).height(3),
            ],
            [Constraint::Length(3)],
        )
        .highlight_style(Style::default().bg(Color::LightBlue));
        match self.mode {
            SidebarMode::Production => self.sidebar_state.select(Some(0)),
            SidebarMode::Buildings => self.sidebar_state.select(Some(1)),
            SidebarMode::Inventory => self.sidebar_state.select(Some(2)),
        };

        frame.render_stateful_widget(table, area, &mut self.sidebar_state);
    }
    fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        match self.mode {
            SidebarMode::Production => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3),      // sidebar
                            Constraint::Percentage(50), // Production
                            Constraint::Fill(1),        // Local market
                        ]
                        .as_ref(),
                    )
                    .split(area);

                self.render_sidebar(frame, chunks[0]);
                self.production_widgets
                    .render(frame, chunks[1], self.current_widget);
                self.lm_widget.render(frame, chunks[2], self.current_widget);
            }
            SidebarMode::Buildings => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3), // sidebar
                            Constraint::Fill(1),   // TODO
                        ]
                        .as_ref(),
                    )
                    .split(area);
                self.render_sidebar(frame, chunks[0]);
                self.render_buildings(frame, chunks[1]);
            }
            SidebarMode::Inventory => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3), // sidebar
                            Constraint::Fill(1),   // TODO
                        ]
                        .as_ref(),
                    )
                    .split(area);
                self.render_sidebar(frame, chunks[0]);
            }
        }
    }

    fn render_buildings(&mut self, frame: &mut Frame, chunk: Rect) {
        self.building_widget
            .render(frame, chunk, self.current_widget);
    }

    /// Returns info about if we need to redresh/redraw and if we're switching planets
    fn handle_input(&mut self, event: Event) -> (NeedRefresh, bool) {
        let mut refresh = NeedRefresh::No;

        refresh = refresh.update(self.lm_widget.handle_input(&event, self.current_widget));
        refresh = refresh.update(
            self.production_widgets
                .handle_input(&event, self.current_widget),
        );

        let Event::Key(KeyEvent {
            code: key,
            modifiers,
            ..
        }) = event
        else {
            return (refresh, false);
        };
        match key {
            KeyCode::Left => {
                if self.current_tab == 0 {
                    self.current_tab = self.planets.len() - 1;
                } else {
                    self.current_tab = self.current_tab.saturating_sub(1);
                }
                (NeedRefresh::APIRefresh, true)
            }
            KeyCode::Right => {
                if self.current_tab == self.planets.len() - 1 {
                    self.current_tab = 0;
                } else {
                    self.current_tab += 1;
                }
                (NeedRefresh::APIRefresh, true)
            }
            KeyCode::Tab => {
                self.current_widget = self.current_widget.next();
                (NeedRefresh::Redraw, false)
            }
            KeyCode::BackTab => {
                self.current_widget = self.current_widget.prev();
                (NeedRefresh::Redraw, false)
            }
            KeyCode::Char('p') if modifiers.contains(KeyModifiers::ALT) => {
                self.mode = SidebarMode::Production;
                (NeedRefresh::Redraw, false)
            }
            KeyCode::Char('b') if modifiers.contains(KeyModifiers::ALT) => {
                self.mode = SidebarMode::Buildings;
                (NeedRefresh::Redraw, false)
            }
            KeyCode::Char('i') if modifiers.contains(KeyModifiers::ALT) => {
                self.mode = SidebarMode::Inventory;
                (NeedRefresh::Redraw, false)
            }

            _ => (refresh, false),
        }
    }
}

/// Runs the main loop of the application.
///
/// Returns when we should exit
async fn run_mainloop(mut terminal: Terminal<impl Backend>, mut app: App) -> anyhow::Result<()> {
    // let client = get_client();
    // let username = client.is_auth().await?;

    let mut needs_redraw = NeedRefresh::APIRefresh;
    let mut last_refresh = Instant::now();
    let mut shared_state = SharedWidgetState::default();

    for p in &app.planets {
        shared_state
            .planet_id_map
            .insert(p.id.clone(), p.name.clone());
    }

    loop {
        let mut switching_planets = false;
        if let Some(event) = get_events()? {
            if let Ok(mut file) = std::fs::File::options()
                .create(true)
                .append(true)
                .open("app.log")
            {
                use std::io::Write;
                writeln!(file, "event {event:?}")?;
            }

            if let Event::Resize(..) = event {
                // At a minimum, we need to redraw the screen
                needs_redraw = needs_redraw.update(NeedRefresh::APIRefresh);
            }
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break;
            }
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('r'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) = event
            {
                needs_redraw = needs_redraw.update(NeedRefresh::APIRefresh);
            }
            let (x, y) = app.handle_input(event);
            needs_redraw = needs_redraw.update(x);
            switching_planets = y;
        }
        if last_refresh.elapsed() > Duration::from_secs(600) {
            needs_redraw = needs_redraw.update(NeedRefresh::APIRefresh);
        }

        if needs_redraw != NeedRefresh::No {
            last_refresh = Instant::now();

            if needs_redraw == NeedRefresh::APIRefresh {
                // before awaiting these calls to .update(), which might take a while, render a frame with a loading message

                if switching_planets {
                    app.lm_widget
                        .switch_planets(&app.planets[app.current_tab].id);
                    app.production_widgets
                        .switch_planets(&app.planets[app.current_tab].id);
                }

                // let jh = tokio::spawn(async {
                //     app.lm_widget.update().await;
                // });

                // jh.await?;

                terminal.draw(|frame| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(0)
                        .constraints(
                            [
                                Constraint::Length(3), // tab selector
                                Constraint::Fill(1),   // tab body
                                Constraint::Length(3), // Details
                            ]
                            .as_ref(),
                        )
                        .split(frame.size());

                    app.render_tabs(frame, chunks[0]);
                    app.render_body(frame, chunks[1]);

                    let area = centered_rect(50, 20, frame.size());
                    let para =
                        Paragraph::new("Loading...").block(Block::default().borders(Borders::ALL));
                    frame.render_widget(Clear, area);
                    frame.render_widget(para, area);
                })?;

                match app.mode {
                    SidebarMode::Production => {
                        app.production_widgets.update(&mut shared_state).await?;
                        app.lm_widget.update(&mut shared_state).await?;
                    }
                    SidebarMode::Buildings => {
                        app.building_widget.update(&mut shared_state).await?;
                    }
                    SidebarMode::Inventory => {
                        // app.inventory_widget.update(&mut shared_state).await?;
                    }
                }
            }

            // terminal.draw(|frame| {
            //     let area = centered_rect(50, 20, frame.size());
            //     let para = Paragraph::new("Loading...")
            //     .block(Block::default().borders(Borders::ALL));
            //     frame.render_widget(Clear, area);
            //     frame.render_widget(para, area);
            // })?;

            terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3), // tab selector
                            Constraint::Fill(1),   // tab body
                            Constraint::Length(3), // status bar
                        ]
                        .as_ref(),
                    )
                    .split(frame.size());

                app.render_tabs(frame, chunks[0]);
                app.render_body(frame, chunks[1]);
            })?;

            if let Ok(mut file) = std::fs::File::options()
                .create(true)
                .append(true)
                .open("app.log")
            {
                use std::io::Write;
                writeln!(file, "Time to render frame: {:?}", last_refresh.elapsed())?;
            }
        }
        needs_redraw = NeedRefresh::No;
    }

    Ok(())
}
