//! Handles interactions with underlying JJ repositories.

use std::{ffi::OsStr, process::Command};

use anyhow::Result;
use bevy::prelude::*;

pub mod log;
pub mod revisions;

pub fn plugin(app: &mut App) {
    app.add_plugins(log::plugin);
}

pub(super) fn execute_jj_command<S: Into<String> + AsRef<OsStr>>(args: Vec<S>) -> Result<String> {
    Ok(String::from_utf8(
        Command::new("jj").args(args.iter()).output()?.stdout,
    )?)
}
