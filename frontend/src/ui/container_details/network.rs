use crate::{api::ContainerDetails, theme::Theme};
use ratzilla::ratatui::{
    style::Style,
    text::{Line, Span},
};

pub(super) fn add_network_info(lines: &mut Vec<Line<'static>>, details: &ContainerDetails) {
    if !details.ports.is_empty() {
        lines.push(Line::from(Span::styled(
            "Ports:",
            Style::default().fg(Theme::YELLOW),
        )));
        for port in &details.ports {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(port.host_port.clone(), Style::default().fg(Theme::TEXT)),
                Span::raw(" → "),
                Span::styled(
                    port.container_port.clone(),
                    Style::default().fg(Theme::TEXT),
                ),
                Span::styled(
                    format!("/{}", port.protocol),
                    Style::default().fg(Theme::DIM),
                ),
            ]));
        }
        lines.push(Line::from(""));
    }

    if !details.networks.is_empty() {
        lines.push(Line::from(Span::styled(
            "Networks:",
            Style::default().fg(Theme::YELLOW),
        )));
        for net in &details.networks {
            lines.push(Line::from(format!("  {}", net)));
        }
        lines.push(Line::from(""));
    }
}
