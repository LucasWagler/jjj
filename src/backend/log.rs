use anyhow::{anyhow, Result};
use bevy::prelude::*;
use regex::{Captures, Match, Regex};

use crate::{app::AppSet, errors, join, screens::Screen};

use super::{execute_jj_command, revisions::Revision};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (read_logs.pipe(errors::forward))
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Interface)),
    );
}

#[derive(Event)]
pub struct LogRequestEvent {
    pub revset: String,
}

impl<S: Into<String>> From<S> for LogRequestEvent {
    fn from(value: S) -> Self {
        Self {
            revset: value.into(),
        }
    }
}

#[derive(Event, Deref, DerefMut)]
pub struct LogResponseEvent(pub Revision);

const LOG_TEMPLATE: &'static str = concat!(
    "'[' ++ ",
    join!(
        " ++ '&JJJ&' ++ ",
        [
            "change_id.shortest()",
            "change_id",
            "commit_id.shortest()",
            "commit_id",
            "divergent",
            "immutable"
        ]
    ),
    " ++ ']'"
);

const MATCH_LOG: &'static str = concat!(
    r#"(?s)\["#,
    join!(
        r#"&JJJ&"#,
        [
            r#"(?<change_id_shortest>.*)"#,
            r#"(?<change_id>.*)"#,
            r#"(?<commit_id_shortest>.*)"#,
            r#"(?<commit_id>.*)"#,
            r#"(?<divergent>.*)"#,
            r#"(?<immutable>.*)"#
        ]
    ),
    r#"\]"#,
);

fn read_logs(
    mut ev_log_request: EventReader<LogRequestEvent>,
    mut ev_log_response: EventWriter<LogResponseEvent>,
) -> Result<()> {
    for LogRequestEvent { revset } in ev_log_request.read() {
        let log = execute_jj_command(vec![
            "log",
            "-r",
            format!("{}", revset).as_str(),
            "-T",
            format!("{}", LOG_TEMPLATE).as_str(),
        ])
        .map_err(|_| anyhow!("Couldn't read log for revset `{revset}`"))?;

        let match_log = Regex::new(MATCH_LOG)
            .map_err(|_| anyhow!("Invalid regular expression: {MATCH_LOG}"))?;

        let mut batch = vec![];

        for (line, caps) in log.lines().map(|line| (line, match_log.captures(line))) {
            let Some(caps) = caps else {
                return Err(anyhow!("Couldn't find captures in line: {line}"));
            };

            let change_id = require(&caps, "change_id")?.as_str().to_string();
            let change_id_shortest = require(&caps, "change_id_shortest")?.as_str().len();

            let commit_id = require(&caps, "commit_id")?.as_str().to_string();
            let commit_id_shortest = require(&caps, "commit_id_shortest")?.as_str().len();

            let description = (caps.name("description"))
                .map(|d| d.as_str().trim().to_string())
                .filter(|d| !d.is_empty());

            let is_divergent = require(&caps, "divergent")?.as_str() == "true";
            let is_immutable = require(&caps, "immutable")?.as_str() == "true";

            batch.push(LogResponseEvent(Revision {
                change_id: (change_id, change_id_shortest),
                commit_id: (commit_id, commit_id_shortest),
                is_divergent,
                is_immutable,
                description,
            }))
        }

        ev_log_response.send_batch(batch);
    }

    Ok(())
}

fn require<'a>(caps: &Captures<'a>, name: &str) -> Result<Match<'a>> {
    caps.name(name)
        .ok_or(anyhow!("Couldn't find `{name}` in captures: {caps:?}"))
}
