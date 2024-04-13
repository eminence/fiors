use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use fiors::{get_material_db, types::Planet, FIOClient};
use once_cell::sync::OnceCell;
use ratatui::{prelude::*, widgets::*};
use widgets::WidgetEnum;

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

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let ret = run_mainloop(terminal).await;

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
    return Ok(None);

    // if event::poll(std::time::Duration::from_millis(500))? {
    //     if let Event::Key(key) = event::read()? {
    //         if key.kind == event::KeyEventKind::Press {
    //             return Ok(Some(key.code));
    //         }
    //     }
    // }
    // Ok(None)
}

struct App {
    client: &'static FIOClient,
    username: String,
    current_tab: usize,
    planets: Vec<Planet>,
    current_widget: WidgetEnum,
    lm_widget: widgets::LocalMarketWidget,
    production_widgets: widgets::ProductionWidget,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum NeedRefresh {
    APIRefresh,
    Redraw,
    No,
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
            lm_widget: widgets::LocalMarketWidget::new(client, &planets[0].id),
            production_widgets: widgets::ProductionWidget::new(client, &username, &planets[0].id),
            client,
            username,
            current_tab: 0,
            current_widget: WidgetEnum::Production,
            planets,
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
    fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(50), // Production
                    Constraint::Fill(1),        // Local market
                ]
                .as_ref(),
            )
            .split(area);

        self.production_widgets
            .render(frame, chunks[0], self.current_widget);
        self.lm_widget.render(frame, chunks[1], self.current_widget);
    }

    fn handle_input(&mut self, event: Event) -> NeedRefresh {
        let mut need_redraw_due_to_widget = false;

        need_redraw_due_to_widget |= self.lm_widget.handle_input(&event, self.current_widget);
        need_redraw_due_to_widget |= self
            .production_widgets
            .handle_input(&event, self.current_widget);
        let Event::Key(KeyEvent { code: key, .. }) = event else {
            if need_redraw_due_to_widget {
                return NeedRefresh::Redraw;
            } else {
                return NeedRefresh::No;
            }
        };
        match key {
            KeyCode::Left => {
                self.current_tab = self.current_tab.saturating_sub(1);
                NeedRefresh::APIRefresh
            }
            KeyCode::Right => {
                self.current_tab = (self.current_tab + 1).min(self.planets.len() - 1);
                NeedRefresh::APIRefresh
            }
            KeyCode::Tab => {
                self.current_widget = self.current_widget.next();
                NeedRefresh::Redraw
            }
            KeyCode::BackTab => {
                self.current_widget = self.current_widget.prev();
                NeedRefresh::Redraw
            }

            _ => {
                if need_redraw_due_to_widget {
                    return NeedRefresh::Redraw;
                } else {
                    return NeedRefresh::No;
                }
            }
        }
    }
}

/// Runs the main loop of the application.
///
/// Returns when we should exit
async fn run_mainloop(mut terminal: Terminal<impl Backend>) -> anyhow::Result<()> {
    // let client = get_client();
    // let username = client.is_auth().await?;

    let mut app = App::new().await?;

    let mut needs_data_refresh = true;
    let mut last_refresh = Instant::now();
    loop {
        let mut needs_redraw = false;
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
                needs_redraw = true;
            }
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break;
            }
            match app.handle_input(event) {
                NeedRefresh::APIRefresh => needs_data_refresh = true,
                NeedRefresh::Redraw => needs_redraw = true,
                NeedRefresh::No => (),
            }
        }
        if last_refresh.elapsed() > Duration::from_secs(600) {
            needs_data_refresh = true;
        }

        if needs_data_refresh || needs_redraw {
            last_refresh = Instant::now();

            if needs_data_refresh {
                // before awaiting these calls to .update(), which might take a while, render a frame with a loading message
                terminal.draw(|frame| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(0)
                        .constraints(
                            [
                                Constraint::Length(3), // tab selector
                                Constraint::Min(0),    // tab body
                                Constraint::Length(1), // status bar
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

                app.lm_widget
                    .switch_planets(&app.planets[app.current_tab].id);
                app.lm_widget.update().await?;

                app.production_widgets
                    .switch_planets(&app.planets[app.current_tab].id)
                    .await;
                app.production_widgets.update().await?;
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
                            Constraint::Min(0),    // tab body
                            Constraint::Length(1), // status bar
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

        needs_data_refresh = false;
    }

    Ok(())
}
