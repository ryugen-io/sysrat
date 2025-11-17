use crate::theme::{ThemeConfig, status_line::StatusLineTheme};
use ratzilla::ratatui::{style::Style, text::Span};

pub fn render_spacer() -> Option<Span<'static>> {
    None
}

pub fn render_text(
    value: String,
    style: Option<&str>,
    theme: &ThemeConfig,
) -> Option<Span<'static>> {
    let s = get_style(style, theme);
    Some(Span::styled(value, s))
}

pub fn render_separator(value: String) -> Option<Span<'static>> {
    Some(Span::raw(value))
}

fn get_style(style_name: Option<&str>, theme: &ThemeConfig) -> Style {
    match style_name {
        Some("label") => StatusLineTheme::label_style(theme),
        Some("value") => StatusLineTheme::value_style(theme),
        Some("error") => StatusLineTheme::error_message_style(theme),
        _ => StatusLineTheme::background(theme),
    }
}
