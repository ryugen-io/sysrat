mod components;
mod config;

use crate::{state::AppState, theme::status_line::StatusLineTheme};
use config::StatusLineConfig;
use ratzilla::ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::Paragraph,
};

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    use ratzilla::ratatui::text::Span;

    // Load embedded config from build time using include_str!
    // Path is relative to this file's location
    let config_toml = include_str!("../../../../sys/layout/statusline.toml");

    // Try to parse, fall back to simple rendering on error
    let config_result: Result<StatusLineConfig, _> = toml::from_str(config_toml);

    if let Err(e) = config_result {
        // Fallback: render error message
        let error_line = Paragraph::new(Line::from(vec![Span::raw(format!(
            "TOML parse error: {}",
            e
        ))]))
        .style(StatusLineTheme::background(&state.current_theme))
        .alignment(Alignment::Left);
        f.render_widget(error_line, area);
        return;
    }

    let config = config_result.unwrap();
    let pane_config = config.get_pane_config(&state.focus);
    let theme = &state.current_theme;

    // Create row constraints dynamically based on config
    let row_count = pane_config.rows.len();
    let constraints: Vec<Constraint> = vec![Constraint::Length(1); row_count];

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    // Render each row
    for (row_idx, row_config) in pane_config.rows.iter().enumerate() {
        let mut spans = vec![];

        // Render each component in the row
        for component_config in &row_config.components {
            if let Some(span) = components::render_component(component_config, state, theme) {
                spans.push(span);
            }
        }

        let line = Paragraph::new(Line::from(spans))
            .style(StatusLineTheme::background(theme))
            .alignment(Alignment::Left);

        f.render_widget(line, rows[row_idx]);
    }
}
