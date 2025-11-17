mod build;
mod state;
mod text;

use super::config::ComponentConfig;
use crate::{state::AppState, theme::ThemeConfig};
use ratzilla::ratatui::text::Span;

pub fn render_component(
    component: &ComponentConfig,
    state: &AppState,
    theme: &ThemeConfig,
) -> Option<Span<'static>> {
    match component {
        ComponentConfig::Spacer => text::render_spacer(),

        ComponentConfig::VimMode => state::render_vim_mode(state, theme),

        ComponentConfig::Filename => state::render_filename(state, theme),

        ComponentConfig::ModifiedIndicator => state::render_modified_indicator(state, theme),

        ComponentConfig::StatusMessage => state::render_status_message(state, theme),

        ComponentConfig::HelpText => state::render_help_text(state, theme),

        ComponentConfig::BuildDate { style } => build::render_build_date(style.as_deref(), theme),

        ComponentConfig::BuildHash { style } => build::render_build_hash(style.as_deref(), theme),

        ComponentConfig::RustVersion { style } => {
            build::render_rust_version(style.as_deref(), theme)
        }

        ComponentConfig::RustEdition { style } => {
            build::render_rust_edition(style.as_deref(), theme)
        }

        ComponentConfig::Dependency { name, style } => {
            build::render_dependency(name, style.as_deref(), theme)
        }

        ComponentConfig::Text { value, style } => {
            text::render_text(value.clone(), style.as_deref(), theme)
        }

        ComponentConfig::Separator { value } => text::render_separator(value.clone()),
    }
}
