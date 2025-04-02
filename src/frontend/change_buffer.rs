use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;
use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Rect, *},
    widgets::Paragraph,
};

use crate::{
    app::AppSet,
    backend::{log::LogResponseEvent, revisions::Revision},
    screens::Screen,
};

use super::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<ChangeBuffer>();

    app.add_systems(OnEnter(Screen::Interface), ChangeBuffer::init);
    app.add_systems(OnExit(Screen::Interface), ChangeBuffer::remove);

    app.add_systems(
        Update,
        (
            read_inputs
                .in_set(AppSet::RecordInput)
                .run_if(in_state(Focus::ChangeBuffer)),
            (purge_change_buffer, read_revisions)
                .chain()
                .in_set(AppSet::Update),
        )
            .run_if(in_state(Screen::Interface)),
    );
}

#[derive(Event)]
pub struct PurgeChangeBufferEvent;

#[derive(Default, Reflect, Resource)]
pub struct ChangeBuffer {
    revisions: Vec<Revision>,
    start_index: usize,
    end_index: Option<usize>,
}

fn read_inputs(mut ev_keypresses: EventReader<KeyEvent>, mut change_buffer: ResMut<ChangeBuffer>) {
    for keypress in ev_keypresses.read() {
        match keypress.code {
            KeyCode::Char('j') => {
                change_buffer.start_index = (change_buffer.revisions.len() - 1)
                    .min(change_buffer.end_index.unwrap_or(change_buffer.start_index) + 1);
                change_buffer.end_index = None;
            }
            KeyCode::Char('k') => {
                change_buffer.start_index = change_buffer.start_index.checked_sub(1).unwrap_or(0);
                change_buffer.end_index = None;
            }
            KeyCode::Char('x') => {
                change_buffer.end_index = (change_buffer.end_index)
                    .map(|i| (change_buffer.revisions.len() - 1).min(i + 1))
                    .or(Some(change_buffer.start_index + 1))
                    .filter(|i| *i != change_buffer.start_index);
            }
            _ => {}
        }
    }
}

fn purge_change_buffer(
    mut ev_purge: EventReader<PurgeChangeBufferEvent>,
    mut change_buffer: ResMut<ChangeBuffer>,
) {
    for _ in ev_purge.read() {
        change_buffer.revisions.clear();
    }
}

fn read_revisions(
    mut ev_log_response: EventReader<LogResponseEvent>,
    mut change_buffer: ResMut<ChangeBuffer>,
) {
    for LogResponseEvent(revision) in ev_log_response.read() {
        change_buffer.revisions.push(revision.clone());
    }
}

impl ResourceWidget for ChangeBuffer {}
impl Widget for &ChangeBuffer {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [revs, empty] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(self.revisions.len() as u16),
                Constraint::Fill(1),
            ])
            .areas(area);

        Paragraph::new(
            (self.revisions.iter().enumerate())
                .map(|(i, rev)| {
                    revision_line(
                        rev,
                        match self.end_index {
                            Some(end_index) => self.start_index <= i && i <= end_index,
                            None => i == self.start_index,
                        },
                    )
                })
                .collect::<Vec<_>>(),
        )
        .scroll((self.end_index.unwrap_or(self.start_index) as u16, 0))
        .render(revs, buf);
        EmptyBuffer.render(empty, buf);
    }
}

fn revision_line(revision: &Revision, is_selected: bool) -> Line {
    let (change_id_prefix, change_id_postfix) =
        revision.change_id.0[..8].split_at(revision.change_id.1);
    let (commit_id_prefix, commit_id_postfix) =
        revision.commit_id.0[..8].split_at(revision.commit_id.1);

    Line::from(vec![
        Span::styled(
            if is_selected { " >> " } else { "    " },
            Style::new().light_cyan(),
        ),
        Span::styled(change_id_prefix, Style::new().not_dim().light_magenta()),
        Span::styled(change_id_postfix, Style::new().dim()),
        Span::from(" "),
        (revision.description.clone())
            .and_then(|d| d.lines().nth(0).map(|d| d.to_string()))
            .map(|d| Span::from(d))
            .unwrap_or_else(|| Span::styled("(empty)", Style::new().yellow())),
        Span::from(" "),
        Span::styled(commit_id_prefix, Style::new().not_dim().light_cyan()),
        Span::styled(commit_id_postfix, Style::new().dim()),
    ])
}
