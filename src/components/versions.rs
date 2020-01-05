use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};

use crate::constants::*;

pub fn render<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title(VERSIONS_TEXT).borders(Borders::ALL);

    let text = [
        Text::styled(
            "Software Version    ",
            Style::default().modifier(Modifier::BOLD),
        ),
        Text::raw("1.2.3\n"),
        Text::styled(
            "Protocol Version    ",
            Style::default().modifier(Modifier::BOLD),
        ),
        Text::raw("1.2.3\n"),
        Text::styled(
            "Wallet Version      ",
            Style::default().modifier(Modifier::BOLD),
        ),
        Text::raw("1.2.3\n"),
    ];

    Paragraph::new(text.iter())
        .block(block.clone())
        .alignment(Alignment::Left)
        .render(f, area);
}
