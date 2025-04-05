use bevy::prelude::*;
use ratatui::prelude::*;

#[derive(Component, Default)]
pub struct Node {
    pub direction: Option<Direction>,
    pub width: Option<Constraint>,
    pub height: Option<Constraint>,
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

/// The base TUI element for the entire widget tree. Only computed elements
/// under this entity will be rendered.
#[derive(Component)]
pub struct ComputedTuiRoot;

/// ComputedTuiRoot
/// 	ComputedLayer (interface layer)
/// 		ComputedLayout (arrange top level components)
/// 			ComputedLayout (revision buffer)
/// 				ComputedLayout (revision line)
/// 					ComputedWidget (left details)
/// 					ComputedWidget (right details)
/// 			ComputedWidget (status line)
/// 			ComputedWidget (command line)
/// 	ComputedLayer (popup layer)
/// 		ComputedLayout (center horizontally)
/// 			ComputedLayout (center vertically)
/// 				ComputedWidget (render popup)
#[derive(Component)]
pub struct ComputedLayer;

#[derive(Component, Deref, DerefMut)]
pub struct ComputedLayout(Layout);

#[derive(Component, Deref)]
pub struct ComputedWidget(Box<dyn Widget + Sync + Send>);
