use anyhow::Context;
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use fiors::get_material_db;
use fiors::FIOClient;
use std::io;

struct Grid<T> {
    cells: Vec<Vec<T>>,
    cols: usize,
}

impl<T> Grid<T> {
    fn new(cols: usize) -> Self {
        Self {
            cells: Vec::new(),
            cols,
        }
    }

    fn push(&mut self, item: T) {
        if self.cells.is_empty() {
            self.cells.push(Vec::new());
        }

        let last = self.cells.last_mut().unwrap();
        last.push(item);

        if last.len() == self.cols {
            self.cells.push(Vec::new());
        }
    }

    fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.cells.iter().map(Vec::as_slice)
    }
}

struct InvItem {
    name: String,
    amount: u32,
    bg_color: Color,
    fg_color: Color,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::args()
        .nth(1)
        .or(std::env::var("FIO_AUTH_TOKEN").ok())
        .unwrap();
    let client = FIOClient::new_with_key(api_key);

    let username = client.is_auth().await?;

    let planets = client.get_storage_planets_for_user(&username).await?;

    for planet in planets {
        let inv = client
            .get_storage_for_user(&username, &planet.id)
            .await?
            .context("No inventory found")?;

        println!("==========");
        println!(" {} ", planet.name.to_ascii_uppercase());
        println!("----------");
        println!();

        // we want an approximate square grid:
        let cols = (inv.items.len() as f32).sqrt().ceil() as usize;
        let mut grid = Grid::new(cols);

        let mut sorted_inv = inv.items.values().collect::<Vec<_>>();
        sorted_inv.sort_by_key(|item| {
            let matinfo = get_material_db()
                .get(item.ticker.as_str())
                .expect("Unknown item");
            matinfo.category
        });

        for item in sorted_inv {
            let matinfo = get_material_db()
                .get(item.ticker.as_str())
                .expect("Unknown item");

            let bg_color = matinfo.category.get_bg_color();
            let fg_color = matinfo.category.get_fg_color();

            let bg_color = crossterm::style::Color::Rgb {
                r: bg_color.0,
                g: bg_color.1,
                b: bg_color.2,
            };
            let fg_color = crossterm::style::Color::Rgb {
                r: fg_color.0,
                g: fg_color.1,
                b: fg_color.2,
            };

            grid.push(InvItem {
                name: item.ticker.clone(),
                amount: item.quantity,
                bg_color,
                fg_color,
            });
        }

        for row in grid.iter_rows() {
            for item in row {
                let bg_color = item.bg_color;
                let fg_color = item.fg_color;

                execute!(
                    io::stdout(),
                    ResetColor,
                    Print(" "),
                    SetForegroundColor(fg_color),
                    SetBackgroundColor(bg_color),
                    Print(format!("{:^5}", item.name)),
                    ResetColor,
                    Print(" "),
                )?;

                // print!(" {:^4} ", item.name);
            }
            println!();
            for item in row {
                let bg_color = item.bg_color;
                let fg_color = item.fg_color;

                execute!(
                    io::stdout(),
                    ResetColor,
                    Print(" "),
                    SetForegroundColor(fg_color),
                    SetBackgroundColor(bg_color),
                    Print(if item.amount < 999 {
                        format!("{:^5}", item.amount)
                    } else if item.amount < 9999 {
                        format!("{:.1}k ", item.amount as f32 / 1000.0)
                    } else {
                        format!(" {:.0}k ", item.amount as f32 / 1000.0)
                    }),
                    ResetColor,
                    Print(" "),
                )?;
            }
            println!("\n");
        }
    }

    // let mat = fiors::materials::MaterialCategory::ConsumablesBasic;

    Ok(())
}
