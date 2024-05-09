use std::{
    fs::File,
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
use ratatui_macros::{horizontal, vertical};
use tracing::{info, level_filters::LevelFilter, trace};
use widgets::{SharedWidgetState, WidgetEnum};

static CLIENT: OnceCell<FIOClient> = OnceCell::new();

mod widgets;

const HELP_TEXT_KEY_STYLE: ratatui::style::Style =
    Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD);

fn get_client() -> &'static FIOClient {
    let api_key = std::env::args()
        .nth(1)
        .or(std::env::var("FIO_AUTH_TOKEN").ok())
        .unwrap();
    CLIENT.get_or_init(|| {
        FIOClient::new_with_key(api_key)
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_ansi(false)
        .with_writer(File::create("app.log")?)
        .with_max_level(LevelFilter::TRACE)
        .init();
    // let client = get_client();
    info!("Starting up...");

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SidebarMode {
    Production,
    Buildings,
    Inventory,
    Debug,
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
    inventory_widget: widgets::InventoryWidget,
    debug_widget: widgets::DebugWidget,
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
            debug_widget: widgets::DebugWidget::new(),
            inventory_widget: widgets::InventoryWidget::new(client, &username, &planets[0].id),
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
                Row::new(vec!["\nDBG\n"]).height(3),
            ],
            [Constraint::Length(3)],
        )
        .highlight_style(Style::default().bg(Color::Blue));
        match self.mode {
            SidebarMode::Production => self.sidebar_state.select(Some(0)),
            SidebarMode::Buildings => self.sidebar_state.select(Some(1)),
            SidebarMode::Inventory => self.sidebar_state.select(Some(2)),
            SidebarMode::Debug => self.sidebar_state.select(Some(3)),
        };

        frame.render_stateful_widget(table, area, &mut self.sidebar_state);
    }
    fn render_help(&self, frame: &mut Frame, area: Rect, help_text: Vec<Span<'static>>) {
        let line = Line::from(help_text);
        let help = Paragraph::new(line).wrap(Wrap { trim: true });

        frame.render_widget(help, area);
    }

    fn render_body(&mut self, frame: &mut Frame, area: Rect, help_text: &[Span<'static>]) {
        let mut help_text = help_text.to_vec();

        let [main_body, footer_area] = vertical![>=1, ==3]
            .margin(0)
            .split(area)
            .to_vec()
            .try_into()
            .unwrap();

        match self.mode {
            SidebarMode::Production => {
                let [planet_area, main_area] = vertical![==3, >=1]
                    .split(main_body)
                    .to_vec()
                    .try_into()
                    .unwrap();

                self.render_tabs(frame, planet_area);

                let [x, y] = horizontal![==1/2, ==1/2]
                    .split(main_area)
                    .to_vec()
                    .try_into()
                    .unwrap();

                self.production_widgets
                    .render(frame, x, self.current_widget);
                self.lm_widget.render(frame, y, self.current_widget);
            }
            SidebarMode::Buildings => {
                let [planet_area, main_area] = vertical![==3, >=1]
                    .split(main_body)
                    .to_vec()
                    .try_into()
                    .unwrap();

                self.render_tabs(frame, planet_area);
                self.building_widget
                    .render(frame, main_area, self.current_widget);
            }
            SidebarMode::Inventory => {
                let [planet_area, main_area] = vertical![==3, >=1]
                    .split(main_body)
                    .to_vec()
                    .try_into()
                    .unwrap();

                self.render_tabs(frame, planet_area);

                self.inventory_widget
                    .render(frame, main_area, self.current_widget);
            }
            SidebarMode::Debug => {
                self.debug_widget.render(frame, main_body);
            }
        }

        match self.mode {
            SidebarMode::Production | SidebarMode::Buildings | SidebarMode::Inventory => {
                help_text.extend(vec![
                    Span::raw("Press "),
                    Span::styled("left", HELP_TEXT_KEY_STYLE),
                    Span::raw(" and "),
                    Span::styled("right", HELP_TEXT_KEY_STYLE),
                    Span::raw(" to switch bases. "),
                ]);
            }
            _ => {}
        }

        help_text.extend(vec![
            Span::styled("Alt-up", HELP_TEXT_KEY_STYLE),
            Span::raw(" and "),
            Span::styled("Alt-down", HELP_TEXT_KEY_STYLE),
            Span::raw(" to switch modes, "),
            Span::styled("q", HELP_TEXT_KEY_STYLE),
            Span::raw(" to quit."),
        ]);
        self.render_help(frame, footer_area, help_text);
    }

    /// Returns info about if we need to redresh/redraw and if we're switching planets
    fn handle_input(&mut self, event: Event) -> (NeedRefresh, bool) {
        let mut refresh = NeedRefresh::No;

        match self.mode {
            SidebarMode::Production => {
                refresh = refresh.update(self.lm_widget.handle_input(&event, self.current_widget));
                refresh = refresh.update(
                    self.production_widgets
                        .handle_input(&event, self.current_widget),
                );
            }
            SidebarMode::Buildings => {
                refresh = refresh.update(
                    self.building_widget
                        .handle_input(&event, self.current_widget),
                );
            }
            SidebarMode::Inventory => {
                refresh = refresh.update(
                    self.inventory_widget
                        .handle_input(&event, self.current_widget),
                );
            }
            SidebarMode::Debug => {}
        }

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
                self.current_widget = WidgetEnum::Production;
                (NeedRefresh::APIRefresh, true)
            }
            KeyCode::Char('b') if modifiers.contains(KeyModifiers::ALT) => {
                self.mode = SidebarMode::Buildings;
                self.current_widget = WidgetEnum::Buildings;
                (NeedRefresh::APIRefresh, true)
            }
            KeyCode::Char('i') if modifiers.contains(KeyModifiers::ALT) => {
                self.mode = SidebarMode::Inventory;
                self.current_widget = WidgetEnum::Inventory;
                (NeedRefresh::APIRefresh, true)
            }
            KeyCode::Char('d') if modifiers.contains(KeyModifiers::ALT) => {
                self.mode = SidebarMode::Debug;
                (NeedRefresh::APIRefresh, true)
            }
            KeyCode::Down if modifiers.contains(KeyModifiers::ALT) => {
                match self.mode {
                    SidebarMode::Production => {
                        self.mode = SidebarMode::Buildings;
                        self.current_widget = WidgetEnum::Buildings;
                    }
                    SidebarMode::Buildings => {
                        self.mode = SidebarMode::Inventory;
                        self.current_widget = WidgetEnum::Inventory;
                    }
                    SidebarMode::Inventory => {
                        self.mode = SidebarMode::Debug;
                    }
                    SidebarMode::Debug => {
                        self.mode = SidebarMode::Production;
                        self.current_widget = WidgetEnum::Production;
                    }
                }
                (NeedRefresh::APIRefresh, true)
            }
            KeyCode::Up if modifiers.contains(KeyModifiers::ALT) => {
                match self.mode {
                    SidebarMode::Production => {
                        self.mode = SidebarMode::Debug;
                    }
                    SidebarMode::Buildings => {
                        self.mode = SidebarMode::Production;
                        self.current_widget = WidgetEnum::Production;
                    }
                    SidebarMode::Inventory => {
                        self.mode = SidebarMode::Buildings;
                        self.current_widget = WidgetEnum::Buildings;
                    }
                    SidebarMode::Debug => {
                        self.mode = SidebarMode::Inventory;
                        self.current_widget = WidgetEnum::Inventory;
                    }
                }
                (NeedRefresh::APIRefresh, true)
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
        if last_refresh.elapsed() > Duration::from_secs(3600) {
            needs_redraw = needs_redraw.update(NeedRefresh::APIRefresh);
        }

        if needs_redraw != NeedRefresh::No {
            last_refresh = Instant::now();

            if needs_redraw == NeedRefresh::APIRefresh {
                // before awaiting these calls to .update(), which might take a while, render a frame with a loading message

                if switching_planets {
                    let planet_id = &app.planets[app.current_tab].id;
                    app.lm_widget.switch_planets(planet_id);
                    app.production_widgets.switch_planets(planet_id);
                    app.building_widget.switch_planets(planet_id);
                    app.inventory_widget.switch_planets(planet_id);
                }

                // let jh = tokio::spawn(async {
                //     app.lm_widget.update().await;
                // });

                // jh.await?;

                terminal.draw(|frame| {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .margin(0)
                        .constraints(
                            [
                                Constraint::Length(4), // Left sidebar
                                Constraint::Fill(1),   // main body
                            ]
                            .as_ref(),
                        )
                        .split(frame.size());

                    app.render_sidebar(frame, chunks[0]);
                    app.render_body(frame, chunks[1], &shared_state.help_text);

                    let area = centered_rect(50, 20, frame.size());
                    let para =
                        Paragraph::new("Loading...").block(Block::default().borders(Borders::ALL));
                    frame.render_widget(Clear, area);
                    frame.render_widget(para, area);
                })?;

                shared_state.help_text.clear();
                match app.mode {
                    SidebarMode::Production => {
                        app.production_widgets.update(&mut shared_state).await?;
                        app.lm_widget.update(&mut shared_state).await?;
                    }
                    SidebarMode::Buildings => {
                        app.building_widget.update(&mut shared_state).await?;
                    }
                    SidebarMode::Inventory => {
                        app.inventory_widget.update(&mut shared_state).await?;
                    }
                    SidebarMode::Debug => {
                        app.debug_widget.update(&mut shared_state).await?;
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
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(4), // Left sidebar
                            Constraint::Fill(1),   // main body
                        ]
                        .as_ref(),
                    )
                    .split(frame.size());

                app.render_sidebar(frame, chunks[0]);
                app.render_body(frame, chunks[1], &shared_state.help_text);
            })?;
        }
        needs_redraw = NeedRefresh::No;
    }

    Ok(())
}
