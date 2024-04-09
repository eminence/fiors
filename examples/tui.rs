use std::{
    collections::HashMap,
    io::{self, stdout},
    time::{Duration, Instant},
};

use anyhow::Context;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use fiors::{
    get_material_db,
    types::{LocalMarket, Planet},
    FIOClient,
};
use once_cell::sync::OnceCell;
use ratatui::{prelude::*, widgets::*};
use syn::token::In;

static CLIENT: OnceCell<FIOClient> = OnceCell::new();

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
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let ret = run_mainloop(terminal).await;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

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
    current_widget: usize,
    table_states: HashMap<usize, TableState>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum NeedsAPIRefresh {
    Yes,
    No,
}

fn get_style_for_material(ticker: &str) -> Style {
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

type Renderer = Box<dyn FnOnce(&mut Frame<'_>, Rect, &mut App)>;

impl App {
    async fn new() -> anyhow::Result<Self> {
        let client = get_client();
        let username = client.is_auth().await?;

        let planets = client.get_storage_planets_for_user(&username).await?;

        Ok(Self {
            client,
            username,
            current_tab: 0,
            current_widget: 0,
            planets,
            table_states: HashMap::new(),
        })
    }

    async fn render_tabs(&self) -> Renderer {
        let tab_names = self.planets.iter().map(|p| p.name.clone());
        let selected = self.current_tab;
        let tabs = Tabs::<'static>::new(tab_names)
            .block(Block::default().title("Bases").borders(Borders::ALL))
            .select(selected);

        let c = move |frame: &mut Frame, area: Rect, app: &mut App| {
            frame.render_widget(tabs, area);
        };

        Box::new(c)
    }
    async fn render_body(&mut self) -> anyhow::Result<Renderer> {
        // let planet = &self.planets[self.current_tab];

        let production = self.render_production_widget().await?;

        let lm_widget = self.get_local_market_widget().await?;
        // self.render_local_market(handle, frame, area)?;

        Ok(Box::new(move |frame: &mut Frame, area, app: &mut App| {
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

            production(frame, chunks[0], app);
            lm_widget(frame, chunks[1], app);
            // frame.render_widget(lm_widget, chunks[1]);

            // let production = Paragraph::new("Production")
            //     .block(Block::default().title("Production").borders(Borders::ALL));
            // frame.render_widget(production, chunks[0]);

            // self.render_local_market(handle, frame, area)?;
        }))
    }

    async fn render_production_widget(
        &mut self,
    ) -> anyhow::Result<Renderer> {
        let client = get_client();
        let planet = &self.planets[self.current_tab];

        let mut production_rows = Vec::new();
        let mut consumption_rows = Vec::new();
        let mut needs_rows = Vec::new();

        //A map of our needs, from ticker name to (is_essential, amount_needed)   (where amount_needed is per 3 weeks)
        let mut total_needs: HashMap<String, (bool, f32)> = HashMap::new();

        // get our base inventory for this planet
        let inv = client
            .get_storage_for_user(&self.username, &planet.id)
            .await?
            .context("No inventory found")?;

        let production_lines = client
            .get_planet_production(&self.username, &planet.id)
            .await?;

        // level load across all production lines (negative values indicate daily need)
        let mut total_daily_production = HashMap::new();
        for prod in production_lines {
            // if prod.building_type != "prefabPlant1" { continue }
            // dbg!(&prod);
            let daily = prod.daily_production();
            for (mat, amt) in daily.outputs {
                *total_daily_production.entry(mat).or_insert(0.0) += amt;
            }
            for (mat, amt) in daily.inputs {
                *total_daily_production.entry(mat).or_insert(0.0) -= amt;
            }
        }

        // get our workforce requirements
        let workforce = client
            .get_planet_workforce_for_user(&self.username, &planet.id)
            .await?;

        for (_workforce_type, details) in workforce.details {
            for need in details.needs {
                // let entry = total_needs.entry(need.ticker.clone()).or_default();
                // (*entry).0 = need.essential;
                // (*entry).1 += need.units_per_interval * num_days_inventory;
                if need.units_per_interval > 0.0 {
                    *total_daily_production.entry(need.ticker).or_insert(0.0) -=
                        need.units_per_interval;
                }
            }
        }

        let total_daily_production: Vec<_> = {
            let mut v: Vec<_> = total_daily_production.into_iter().collect();
            v.sort_by(|a, b| {
                let a_cat = get_material_db().get(a.0.as_str()).unwrap().category;
                let b_cat = get_material_db().get(b.0.as_str()).unwrap().category;

                a_cat.cmp(&b_cat).then(a.0.cmp(&b.0))
            });
            v
        };

        for (material, amount) in &total_daily_production {
            if *amount > 0.0 {
                // let colored_material = MaterialWithColor::new(&material);
                // println!(
                //     "  Producing {} per day",
                //     colored_material.with_amount(amount.round() as i32),
                // );
                production_rows.push(Row::new(vec![
                    Span::raw("Producing"),
                    Span::raw(format!("{amount:.1}")),
                    Span::raw(format!("{}", material)).style(get_style_for_material(&material)),
                    Span::raw("per day"),
                ]));
            } else {
                let inv_amount = inv.items.get(&*material).map(|i| i.quantity).unwrap_or(0);
                let days = inv_amount as f32 / -amount;

                consumption_rows.push(Row::new(vec![
                    Span::raw("Consuming"),
                    Span::raw(format!("{:.1}", -amount)),
                    Span::raw(format!("{}", material)).style(get_style_for_material(&material)),
                    Span::raw("per day"),
                    Span::raw("lasting"),
                    Span::raw(format!("{:.1} days", days)).style(get_style_for_days(days)),
                ]));

                // assuming we want 21 days worth of materials, how much should we buy?
                let amount_in_inventory =
                    inv.items.get(&*material).map(|i| i.quantity).unwrap_or(0);
                let target_amount = -amount * 21.0;
                let amount_to_buy = target_amount - amount_in_inventory as f32;
                if amount_to_buy > 0.0 {
                    // are any other bases producing a surplus of this material?
                    // TODO

                    needs_rows.push(Row::new(vec![
                        Span::raw(format!("{:.1}", amount_to_buy)),
                        Span::raw(format!("{}", material)).style(get_style_for_material(&material)),
                    ]));
                }
            }
        }

        let production_table = Table::new(
            production_rows,
            [
                Constraint::Length(9), // "Producing"
                Constraint::Length(4), // amount
                Constraint::Length(3), // ticker
                Constraint::Length(7), // "Per day"
            ],
        )
        .block(
            Block::default()
                .title("Net Production")
                .border_style(style::Style::default().fg(if self.current_widget == 0 {
                    Color::Cyan
                } else {
                    Color::White
                }))
                .borders(Borders::ALL),
        );

        // now that we know the table length, make sure the selected row is properly bounded
        if let Some(state) = self.table_states.get_mut(&2) {
            if state.selected().unwrap_or(0) >= consumption_rows.len() {
                state.select(Some(consumption_rows.len() - 1));
            }
        }
        let consumption_table = Table::new(
            consumption_rows,
            [
                Constraint::Length(9), // "Consuming"
                Constraint::Length(4), // amount
                Constraint::Length(3), // ticker
                Constraint::Length(7), // "Per day"
                Constraint::Length(7), // "lasting"
                Constraint::Fill(1),   // days
            ],
        )
        .highlight_symbol(">")
        .block(
            Block::default()
                .title("Net Consumption")
                .border_style(style::Style::default().fg(if self.current_widget == 2 {
                    Color::Cyan
                } else {
                    Color::White
                }))
                .borders(Borders::ALL),
        );

        let needs_table = Table::new(
            needs_rows,
            [
                Constraint::Length(6), // amount
                Constraint::Length(3), // ticker
            ],
        )
        .block(
            Block::default()
                .title("Needs to acquire")
                .border_style(style::Style::default().fg(if self.current_widget == 4 {
                    Color::Cyan
                } else {
                    Color::White
                }))
                .borders(Borders::ALL),
        );

        Ok(Box::new(move |frame: &mut Frame, area, app: &mut App| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Ratio(1, 3), // production
                        Constraint::Ratio(1, 3), // consumption
                        Constraint::Ratio(1, 3), // needs
                    ]
                    .as_ref(),
                )
                .split(area);

            frame.render_widget(production_table, chunks[0]);
            frame.render_stateful_widget(consumption_table, chunks[1], app.table_states.entry(2).or_default());
            frame.render_widget(needs_table, chunks[2]);
        }))
    }

    async fn get_local_market_widget(
        &self,
    ) -> anyhow::Result<Renderer> {
        let planet = &self.planets[self.current_tab];

        let lm = self.client.get_planet_localmarket(&planet.id).await?;
        let mut our_selling_orders = Vec::new();

        let mut notes = Vec::new();
        let mut rows = Vec::new();

        for ad in &lm.selling_ads {
            let ticker_style = get_style_for_material(&ad.material_ticker);

            let mut symbol = " ";
            if ad.creator_company_code == "EM32" {
                our_selling_orders.push(ad.material_ticker.clone());
                continue;
            }

            let cx = self
                .client
                .get_exchange_info(&format!("{}.CI1", ad.material_ticker))
                .await?;

            let price_per_unit = ad.total_price / ad.material_amount as f32;
            if price_per_unit < cx.ask.unwrap_or(cx.price) {
                symbol = "+";
                notes.push(Line::from(vec![
                    Span::raw("Good deal on "),
                    Span::raw(format!("{}", ad.material_ticker)).style(ticker_style),
                    Span::raw(" if we need any"),
                ]));
            }

            // Could we buy this local market order and instantly sell it on the CX for a profit?
            let instant = cx.instant_sell(ad.material_amount);
            if let Some(instant) = instant {
                if instant.total_value > ad.total_price {
                    symbol = "!";
                    notes.push(Line::from(vec![
                        Span::raw("Can by "),
                        Span::raw(format!("{}", ad.material_ticker)).style(ticker_style),
                        Span::raw(format!(
                            " for {} and instantly sell for {}",
                            ad.total_price, instant.total_value
                        )),
                    ]));
                }
            }

            let cx_ask = cx
                .ask
                .map(|b| format!("{:.0}%", 100.0 * price_per_unit / b))
                .unwrap_or("N/A".into());

            rows.push(Row::new(vec![
                Span::raw(symbol),
                Span::raw(format!("{}", ad.creator_company_name)),
                Span::raw("selling"),
                Span::raw(ad.material_amount.to_string()),
                Span::raw(format!("{}", ad.material_ticker))
                    .style(get_style_for_material(&ad.material_ticker)),
                Span::raw(format!("{:.1} {}", ad.total_price, ad.currency)),
                Span::raw(format!("{:.1}/u", price_per_unit)),
                Span::raw(cx_ask),
            ]));
        }
        for ad in &lm.buying_ads {
            let ticker_style = get_style_for_material(&ad.material_ticker);
            let mut symbol = " ";
            let price_per_unit = ad.total_price / ad.material_amount as f32;
            let cx = self
                .client
                .get_exchange_info(&format!("{}.CI1", ad.material_ticker))
                .await?;
            if price_per_unit > cx.bid.unwrap_or(cx.price) {
                notes.push(Line::raw(format!("Good deal on {}", ad.material_ticker)));
                symbol = "+";
            }

            let cx_bid = cx
                .bid
                .map(|b| format!("{:.0}%", 100.0 * price_per_unit / b))
                .unwrap_or("N/A".into());

            rows.push(Row::new(vec![
                Span::raw(symbol),
                Span::raw(format!("{}", ad.creator_company_name)),
                Span::raw("buying"),
                Span::raw(ad.material_amount.to_string()),
                Span::raw(format!("{}", ad.material_ticker)).style(ticker_style),
                Span::raw(format!("{:.1} {}", ad.total_price, ad.currency)),
                Span::raw(format!("{:.1}/u", price_per_unit)),
                Span::raw(cx_bid),
            ]));
        }

        // let paragraph = Paragraph::new(text)
        //     .block(Block::default().title("Local Market").borders(Borders::ALL));
        let table = Table::new(
            rows,
            [
                Constraint::Length(1), // symbol
                Constraint::Fill(2),   // company
                Constraint::Length(7), // action
                Constraint::Length(5), // amount
                Constraint::Length(3), // ticker
                Constraint::Fill(1),   // total price
                Constraint::Fill(1),   // ppu
                Constraint::Length(4), // percent of CX
            ],
        )
        .header(
            Row::new(vec![
                "", "Company", "Action", "Amt", "Tck", "Price", "PPU", "CX %",
            ])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .title("Local Market")
                .border_style(style::Style::default().fg(if self.current_widget == 1 {
                    Color::Cyan
                } else {
                    Color::White
                }))
                .borders(Borders::ALL),
        );

        // construct paragraph to hold other info
        let paragraph = Paragraph::new(notes).block(
            Block::default()
                .title("LM notes")
                .border_style(style::Style::default().fg(if self.current_widget == 3 {
                    Color::Cyan
                } else {
                    Color::White
                }))
                .borders(Borders::ALL),
        );

        Ok(Box::new(move |frame: &mut Frame, area, app: &mut App| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Fill(1), // table
                        Constraint::Fill(1), // paragraph
                    ]
                    .as_ref(),
                )
                .split(area);

            frame.render_widget(table, chunks[0]);
            frame.render_widget(paragraph, chunks[1]);
        }))
    }

    fn handle_input(&mut self, key: KeyCode) -> NeedsAPIRefresh {
        match key {
            KeyCode::Left => {
                self.current_tab = self.current_tab.saturating_sub(1);
                NeedsAPIRefresh::Yes
            }
            KeyCode::Right => {
                self.current_tab = (self.current_tab + 1).min(self.planets.len() - 1);
                NeedsAPIRefresh::Yes
            }
            KeyCode::Tab => {
                self.current_widget = (self.current_widget + 1) % 5;
                NeedsAPIRefresh::Yes
            }
            KeyCode::BackTab => {
                if self.current_widget == 0 {
                    self.current_widget = 4;
                } else {
                    self.current_widget -= 1;
                }
                NeedsAPIRefresh::Yes
            }
            KeyCode::Down => {
                let state = self.table_states.entry(self.current_widget).or_default();
                let selected = state.selected().unwrap_or(0);
                state.select(Some(selected + 1));
                NeedsAPIRefresh::Yes
            }
            KeyCode::PageDown => {
                let state = self.table_states.entry(self.current_widget).or_default();
                let selected = state.selected().unwrap_or(0);
                state.select(Some(selected + 5));
                NeedsAPIRefresh::Yes
            }
            KeyCode::Up => {
                let state = self.table_states.entry(self.current_widget).or_default();
                let selected = state.selected().unwrap_or(0);
                state.select(Some(selected.saturating_sub(1)));
                NeedsAPIRefresh::Yes
            }
            KeyCode::PageUp => {
                let state = self.table_states.entry(self.current_widget).or_default();
                let selected = state.selected().unwrap_or(0);
                state.select(Some(selected.saturating_sub(5)));
                NeedsAPIRefresh::Yes
            }
            _ => NeedsAPIRefresh::No,
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
            if let Event::Resize(..) = event {
                needs_redraw = true;
            }
            if let Event::Key(KeyEvent { code, .. }) = event {
                if code == KeyCode::Char('q') {
                    break;
                }
                if app.handle_input(code) == NeedsAPIRefresh::Yes {
                    needs_data_refresh = true;
                }
            }
        }

        if needs_data_refresh || needs_redraw || last_refresh.elapsed() > Duration::from_secs(60) {
            last_refresh = Instant::now();
            // terminal.draw(|frame| {
            //     let area = centered_rect(50, 20, frame.size());
            //     let para = Paragraph::new("Loading...")
            //     .block(Block::default().borders(Borders::ALL));
            //     frame.render_widget(Clear, area);
            //     frame.render_widget(para, area);
            // })?;

            let tabs = app.render_tabs().await;
            let body = app.render_body().await?;
            terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3), // tab selector
                            Constraint::Min(0),    // tab body
                        ]
                        .as_ref(),
                    )
                    .split(frame.size());

                tabs(frame, chunks[0], &mut app);
                body(frame, chunks[1], &mut app);
            })?;
        }

        needs_data_refresh = false;
    }

    Ok(())
}
