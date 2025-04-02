use bevy::prelude::*;

#[derive(Clone, Debug, Reflect)]
pub struct Revision {
    pub change_id: (String, usize),
    pub commit_id: (String, usize),
    pub description: Option<String>,
    pub is_divergent: bool,
    pub is_immutable: bool,
}
