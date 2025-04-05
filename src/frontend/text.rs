use bevy::prelude::*;
use ratatui::prelude::*;

type RatatuiStyle = ratatui::style::Style;

#[derive(Component, Default, Deref, DerefMut)]
#[require(Style)]
pub struct Span(String);

#[derive(Component, Default)]
pub struct Style(RatatuiStyle);

impl Style {
    pub const fn new() -> Self {
        Self(RatatuiStyle::new())
    }

    pub const fn reset() -> Self {
        Self(RatatuiStyle::reset())
    }

    pub fn fg(self, color: Color) -> Self {
        Self(self.0.fg(color))
    }

    pub const fn bg(self, color: Color) -> Self {
        Self(self.0.bg(color))
    }

    pub const fn underline_color(self, color: Color) -> Self {
        Self(self.0.underline_color(color))
    }

    pub const fn add_modifier(self, modifier: Modifier) -> Self {
        Self(self.0.add_modifier(modifier))
    }

    pub const fn remove_modifier(self, modifier: Modifier) -> Self {
        Self(self.0.remove_modifier(modifier))
    }
}
