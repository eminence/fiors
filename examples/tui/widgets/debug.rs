use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::{SharedWidgetState, WidgetEnum};

pub struct DebugWidget {
    pub lines: Vec<Line<'static>>,
}

impl DebugWidget {
    pub(crate) fn new() -> DebugWidget {
        DebugWidget { lines: Vec::new() }
    }

    pub async fn update(&mut self, shared_state: &mut SharedWidgetState) -> anyhow::Result<()> {
        for line in &shared_state.debug_messages {
            self.lines.push(Line::from(line.to_string()));
        }

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let para = Paragraph::new(self.lines.clone())
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Debug").borders(Borders::ALL));

        frame.render_widget(para, area);
    }
}
