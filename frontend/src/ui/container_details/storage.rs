use crate::{api::ContainerDetails, theme::Theme};
use ratzilla::ratatui::{
    style::Style,
    text::{Line, Span},
};

pub(super) fn add_storage_info(lines: &mut Vec<Line<'static>>, details: &ContainerDetails) {
    if !details.volumes.is_empty() {
        lines.push(Line::from(Span::styled(
            "Volumes:",
            Style::default().fg(Theme::YELLOW),
        )));
        for vol in &details.volumes {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(vol.source.clone(), Style::default().fg(Theme::TEXT)),
                Span::raw(" → "),
                Span::styled(vol.destination.clone(), Style::default().fg(Theme::TEXT)),
                Span::styled(format!(" ({})", vol.mode), Style::default().fg(Theme::DIM)),
            ]));
        }
        lines.push(Line::from(""));
    }
}
