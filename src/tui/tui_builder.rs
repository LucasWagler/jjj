use bevy::prelude::*;

pub trait TuiBuilderCommandsExt {
    fn tui<T>(&mut self, context: T) -> TuiBuilder<T>;
}

impl TuiBuilderCommandsExt for Commands<'_, '_> {
    fn tui<T>(&mut self, context: T) -> TuiBuilder<T> {
        TuiBuilder {
            commands: self.reborrow(),
            context,
        }
    }
}

impl TuiBuilderCommandsExt for EntityCommands<'_> {
    fn tui<T>(&mut self, context: T) -> TuiBuilder<T> {
        TuiBuilder {
            commands: self.commands(),
            context,
        }
    }
}

pub struct TuiBuilder<'a, T> {
    commands: Commands<'a, 'a>,
    context: T,
}

impl<'a, T> TuiBuilder<'a, T> {
    pub fn context(&self) -> &T {
        &self.context
    }

    pub fn commands(&mut self) -> &mut Commands<'a, 'a> {
        &mut self.commands
    }
}

pub struct TuiRoot;

impl TuiBuilder<'_, TuiRoot> {
    pub fn spawn(&mut self, bundle: impl Bundle) -> TuiBuilder<Entity> {
        let entity = self.commands().spawn(bundle).id();
        self.commands.tui(entity)
    }
}

impl<T: TuiId> TuiBuilder<'_, T> {
    pub fn id(&self) -> Entity {
        self.context().id()
    }

    pub fn spawn(&mut self, bundle: impl Bundle) -> TuiBuilder<Entity> {
        let mut entity = Entity::PLACEHOLDER;

        let parent = self.id();
        self.commands().entity(parent).with_children(|parent| {
            entity = parent.spawn(bundle).id();
        });

        self.commands().tui(entity)
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity_commands().insert(bundle);
        self
    }

    pub fn entity_commands(&mut self) -> EntityCommands {
        let entity = self.id();
        self.commands().entity(entity)
    }

    pub fn entity_commands_with<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EntityCommands),
    {
        let mut commands = self.entity_commands();
        f(&mut commands);
        self
    }
}

pub trait TuiId {
    fn id(&self) -> Entity;
}

impl TuiId for Entity {
    fn id(&self) -> Entity {
        *self
    }
}

impl<T> TuiId for (Entity, T) {
    fn id(&self) -> Entity {
        self.0
    }
}
