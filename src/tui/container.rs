use bevy::prelude::*;

use super::prelude::*;

pub trait TuiBuilderContainerExt {
    /// Creates a container to place other elements into.
    ///
    /// ## Example
    ///
    /// ```rust
    /// commands.tui_builder(TuiRoot).container(Node::column(), |root| {
    /// 	root
    /// 		.container(Node::row().height(Constraint::Fill(1)), |buffer| {
    /// 			// ...
    /// 		})
    /// 		.container(Node::row().height(Constraint::Length(1)), |status| {
    /// 			// ...
    /// 		})
    /// 		.container(Node::row().height(Constraint::Length(1)), |command| {
    /// 			// ...
    /// 		});
    /// });
    /// ```
    fn container<F>(&mut self, bundle: impl Bundle, spawn_children: F) -> TuiBuilder<'_, Entity>
    where
        F: FnOnce(&mut TuiBuilder<Entity>);
}

impl TuiBuilderContainerExt for TuiBuilder<'_, TuiRoot> {
    fn container<F>(&mut self, bundle: impl Bundle, spawn_children: F) -> TuiBuilder<'_, Entity>
    where
        F: FnOnce(&mut TuiBuilder<Entity>),
    {
        let mut tui_builder = self.spawn(bundle);
        spawn_children(&mut tui_builder);
        tui_builder
    }
}

impl TuiBuilderContainerExt for TuiBuilder<'_, Entity> {
    fn container<F>(&mut self, bundle: impl Bundle, spawn_children: F) -> TuiBuilder<'_, Entity>
    where
        F: FnOnce(&mut TuiBuilder<Entity>),
    {
        let mut tui_builder = self.spawn(bundle);
        spawn_children(&mut tui_builder);
        tui_builder
    }
}

pub trait TuiBuilderInsertContainerExt<'a> {
    /// Creates a container to place other elements into.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let entity = commands.get_entity(..).unwrap();
    /// commands.tui_builder(entity).insert_container(Node::column(), |root| {
    /// 	root
    /// 		.container(Node::row().height(Constraint::Fill(1)), |buffer| {
    /// 			// ...
    /// 		})
    /// 		.container(Node::row().height(Constraint::Length(1)), |status| {
    /// 			// ...
    /// 		})
    /// 		.container(Node::row().height(Constraint::Length(1)), |command| {
    /// 			// ...
    /// 		});
    /// });
    /// ```
    fn insert_container<F>(
        &'a mut self,
        bundle: impl Bundle,
        spawn_children: F,
    ) -> &'a mut TuiBuilder<'a, Entity>
    where
        F: FnOnce(&mut TuiBuilder<Entity>);
}

impl<'a> TuiBuilderInsertContainerExt<'a> for TuiBuilder<'a, Entity> {
    fn insert_container<F>(
        &'a mut self,
        bundle: impl Bundle,
        spawn_children: F,
    ) -> &mut TuiBuilder<'a, Entity>
    where
        F: FnOnce(&mut TuiBuilder<Entity>),
    {
        let mut tui_builder = self.insert(bundle);
        spawn_children(&mut tui_builder);
        self
    }
}
