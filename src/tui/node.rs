use bevy::prelude::*;
use ratatui::{layout::Spacing, prelude::*};

#[derive(Component, Default)]
pub struct Node {
    pub direction: Option<Direction>,
    pub width: Option<Constraint>,
    pub height: Option<Constraint>,
    pub margin: Margin,
    pub spacing: Spacing,
    pub clear: bool,
}

impl Node {
    pub fn row() -> Self {
        Self {
            direction: Some(Direction::Horizontal),
            ..default()
        }
    }

    pub fn column() -> Self {
        Self {
            direction: Some(Direction::Vertical),
            ..default()
        }
    }

    pub fn width(self, width: Constraint) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }

    pub fn height(self, height: Constraint) -> Self {
        Self {
            height: Some(height),
            ..self
        }
    }
}
