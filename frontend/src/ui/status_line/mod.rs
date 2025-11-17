mod left;
mod right;

use crate::state::AppState;
use ratzilla::ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    // Split status area into 4 rows:
    // Row 0: Empty spacing line
    // Row 1: Status info (mode, file, messages, help)
    // Row 2: Empty spacing line
    // Row 3: Build info (versions, dates)
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Spacing
            Constraint::Length(1), // Status info
            Constraint::Length(1), // Spacing
            Constraint::Length(1), // Build info
        ])
        .split(area);

    // Row 1: Status info - split into left (content) and right (empty for now)
    let status_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100), // Left: status content
            Constraint::Length(0),       // Right: empty (reserved for future use)
        ])
        .split(rows[1]);

    left::render(f, state, status_cols[0]);

    // Row 3: Build info - split into left (content) and right (empty for now)
    let build_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100), // Left: build content
            Constraint::Length(0),       // Right: empty (reserved for future use)
        ])
        .split(rows[3]);

    right::render(f, state, build_cols[0]);
}
