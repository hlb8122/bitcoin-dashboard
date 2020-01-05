use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};

use crate::constants::*;
use crate::components::versions;

pub fn render<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Length(50)].as_ref())
        .split(area);
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(horizontal_chunks[0]);
    versions::render(f, left_chunks[0])
}