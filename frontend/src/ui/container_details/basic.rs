use crate::{api::ContainerDetails, theme::Theme};
use ratzilla::ratatui::{
    style::Style,
    text::{Line, Span},
};

pub(super) fn add_basic_info(lines: &mut Vec<Line<'static>>, details: &ContainerDetails) {
    lines.push(Line::from(vec![
        Span::styled("ID: ", Style::default().fg(Theme::DIM)),
        Span::styled(details.id.clone(), Style::default().fg(Theme::TEXT)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Name: ", Style::default().fg(Theme::DIM)),
        Span::styled(details.name.clone(), Style::default().fg(Theme::TEXT)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Image: ", Style::default().fg(Theme::DIM)),
        Span::styled(details.image.clone(), Style::default().fg(Theme::LAVENDER)),
    ]));
    lines.push(Line::from(""));

    let state_color = match details.state.as_str() {
        "running" => Theme::SUCCESS,
        _ => Theme::PEACH,
    };
    lines.push(Line::from(vec![
        Span::styled("State: ", Style::default().fg(Theme::DIM)),
        Span::styled(details.state.clone(), Style::default().fg(state_color)),
    ]));

    if let Some(health) = &details.health {
        lines.push(Line::from(vec![
            Span::styled("Health: ", Style::default().fg(Theme::DIM)),
            Span::styled(health.clone(), Style::default().fg(Theme::SUCCESS)),
        ]));
    }
    lines.push(Line::from(""));
}
