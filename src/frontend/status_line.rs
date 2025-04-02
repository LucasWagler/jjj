use bevy::prelude::*;
use ratatui::{
    prelude::{Rect, *},
    widgets::Block,
};

use crate::{app::AppSet, backend::log::LogRequestEvent, screens::Screen};

use super::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<StatusLine>();

    app.add_systems(OnEnter(Screen::Interface), StatusLine::init);
    app.add_systems(OnExit(Screen::Interface), StatusLine::remove);

    app.add_systems(
        Update,
        monitor_revset
            .run_if(in_state(Screen::Interface))
            .in_set(AppSet::Update),
    );
}

fn monitor_revset(
    mut ev_log_request: EventReader<LogRequestEvent>,
    mut status_line: ResMut<StatusLine>,
) {
    for LogRequestEvent { revset } in ev_log_request.read() {
        status_line.revset = Some(revset.clone());
    }
}

#[derive(Default, Reflect, Resource)]
pub struct StatusLine {
    pub revset: Option<String>,
}

impl StatusLine {
    fn revset(&self) -> String {
        self.revset.clone().unwrap_or("-".into())
    }
}

impl ResourceWidget for StatusLine {}
impl Widget for &StatusLine {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [_, refset_area] = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(self.revset().len().try_into().unwrap()),
            ])
            .areas(area);

        Block::default().on_white().render(area, buf);
        Span::styled(self.revset(), Style::new().black()).render(refset_area, buf);
    }
}
