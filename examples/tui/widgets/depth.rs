use fiors::types::{MarketOrder, Ticker};
use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Borders},
    Frame,
};

use super::{SharedWidgetState, WidgetEnum};

#[derive(Default)]
pub struct DepthChartWidget {
    buy_orders: Vec<MarketOrder>,
    sell_orders: Vec<MarketOrder>,
    ticker: String,
}

impl DepthChartWidget {
    pub fn update_with_ticker(&mut self, ticker: &Ticker) {
        self.buy_orders = ticker.buying_orders.clone();
        self.buy_orders.sort_by(|a, b| b.item_cost.partial_cmp(&a.item_cost).unwrap());

        self.sell_orders = ticker.selling_orders.clone();
        self.sell_orders.sort_by(|a, b| a.item_cost.partial_cmp(&b.item_cost).unwrap());

        self.ticker = ticker.name.to_string();
        tracing::trace!("Updated depth chart widget with ticker: {}", self.ticker);
    }

    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        Ok(())
    }
    pub fn render(&mut self, frame: &mut Frame, area: Rect, current_widget: WidgetEnum) {
        let data: Vec<Bar> = self.calc(((area.width - 2) / 1) as usize, 0);

        let bar = BarChart::default()
            .bar_gap(0)
            .bar_width(1)
            .data(BarGroup::default().bars(&data))
            .block(
                Block::default()
                    .title(format!("Depth Chart for {}", self.ticker))
                    .borders(Borders::ALL),
            );

        frame.render_widget(bar, area);
    }
    fn calc(&self, width: usize, selected_idx: usize) -> Vec<Bar> {
        /*



          -------------------------------------------
                               ^             ^
                               |             + highested sell price
                               Lowest sell price
        */

        // calculate price spread (stopping at market maker prices, if they exist)
        let mut sell_orders = self
            .sell_orders
            .iter()
            .take_while(|o| o.item_count.is_some());

        let lowest_priced_sell = sell_orders.next().unwrap();
        let hightest_priced_sell = sell_orders.last().unwrap();

        let mut buy_orders = self
            .buy_orders
            .iter()
            .take_while(|o| o.item_count.is_some());

        let highest_priced_buy = buy_orders.next().unwrap();
        let lowest_priced_buy = buy_orders.last().unwrap();

        // dbg!(lowest_priced_buy);
        // dbg!(highest_priced_buy);
        // dbg!(lowest_priced_sell);
        // dbg!(hightest_priced_sell);

        // let inner_spread =
        //     lowest_priced_sell.unwrap().item_cost - highest_price_buy.unwrap().item_cost;
        // dbg!(inner_spread);

        // let outer_spread =
        //     hightest_price_sell.unwrap().item_cost - lowest_price_buy.unwrap().item_cost;
        // dbg!(outer_spread);

        // given our width and our spread, what's the bucket size?
        let half_width = (width as f32 / 2.0).floor() as usize;
        let sell_spread = hightest_priced_sell.item_cost - lowest_priced_sell.item_cost;
        let sell_bucket_size = sell_spread / half_width as f32;
        let buy_spread = highest_priced_buy.item_cost - lowest_priced_buy.item_cost;
        let buy_bucket_size = buy_spread / half_width as f32;

        tracing::trace!(width, half_width, sell_spread, sell_bucket_size);

        let buy_orders: Vec<_> = self
            .buy_orders
            .iter()
            .take_while(|o| o.item_count.is_some())
            .collect();

        let mut b: Vec<_> = (0..half_width)
            .map(|i| {
                let window_max = lowest_priced_buy.item_cost + (buy_bucket_size * (i) as f32);
                let next_window_max =
                    lowest_priced_buy.item_cost + (buy_bucket_size * (i + 1) as f32);
                tracing::trace!(window_max);

                // what's the total volume of orders at above this price?
                let supply = buy_orders
                    .iter()
                    .filter_map(|o| {
                        if o.item_cost >= window_max {
                            Some(o.item_count.unwrap_or_default())
                        } else {
                            None
                        }
                    })
                    .sum::<u32>();

                let our_order_in_window = buy_orders.iter().any(|o| {
                    matches!(&o.company_code, Some(x) if x == "EM32")
                        && o.item_cost >= window_max
                        && o.item_cost < next_window_max
                });
                tracing::trace!(
                    our_order_in_window,
                    window_max,
                    next_window_max,
                    "Total buy orders at or above {window_max}: {supply}"
                );

                Bar::default()
                    .value(supply as u64)
                    .style(Style::default().fg(ratatui::style::Color::LightGreen))
            })
            .collect();

        let sell_orders: Vec<_> = self
            .sell_orders
            .iter()
            .take_while(|o| o.item_count.is_some())
            .collect();

        let s: Vec<_> = (0..half_width)
            .map(|i| {
                let window_max = lowest_priced_sell.item_cost + (sell_bucket_size * (i + 1) as f32);
                let prev_window_max = lowest_priced_sell.item_cost + (sell_bucket_size * i as f32);
                tracing::trace!(window_max);

                let supply = sell_orders
                    .iter()
                    .filter_map(|o| {
                        if o.item_cost <= window_max {
                            Some(o.item_count.unwrap_or_default())
                        } else {
                            None
                        }
                    })
                    .sum::<u32>();

                // do we have any sell orders in this bucket?
                let our_order_in_window = sell_orders.iter().any(|o| {
                    matches!(&o.company_code, Some(x) if x == "EM32")
                        && o.item_cost <= window_max
                        && o.item_cost > prev_window_max
                });

                tracing::trace!(
                    our_order_in_window,
                    prev_window_max,
                    window_max,
                    "Total sell orders at or below {window_max}: {supply}"
                );

                if our_order_in_window {
                    Bar::default()
                        .value(supply as u64)
                        .style(Style::default().fg(ratatui::style::Color::LightRed))
                        .label(Line::from("•"))
                } else {
                    Bar::default()
                        .value(supply as u64)
                        .style(Style::default().fg(ratatui::style::Color::LightRed))
                }
            })
            .collect();

        // trace!(?v);

        b.extend(s);
        b
    }
}
