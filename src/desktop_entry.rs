use freedesktop_desktop_entry::{default_paths, get_languages_from_env, Iter};
use freedesktop_icons::lookup;
use std::env::home_dir;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;

use gpui::{App, SharedString};

#[derive(Debug)]
pub struct DesktopEntry {
    pub name: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<Arc<Path>>,
    pub exec: String,
    pub working_dir: Option<PathBuf>,
}

impl DesktopEntry {
    pub fn open(&self, cx: &mut App) {
        let mut parts = self.exec.split_ascii_whitespace();
        let mut cmd = Command::new(parts.next().unwrap());
        cmd.args(parts);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        if let Some(cwd) = &self.working_dir {
            cmd.current_dir(cwd);
        } else if let Some(cwd) = home_dir() {
            cmd.current_dir(cwd);
        }

        match cmd.spawn() {
            Ok(_) => cx.quit(),
            Err(e) => {
                eprintln!("Failed to launch {}: {} {:?}", self.name, e, self)
            }
        }
    }
}

pub fn get_desktop_entries() -> Vec<DesktopEntry> {
    let locales = get_languages_from_env();

    Iter::new(default_paths())
        .flat_map(|path| freedesktop_desktop_entry::DesktopEntry::from_path(path, Some(&locales)))
        .filter_map(|entry| {
            Some(DesktopEntry {
                name: entry.name(&locales).map(|c| c.into_owned().into())?,
                description: entry.comment(&locales).map(|c| c.into_owned().into()),
                icon: entry
                    .icon()
                    .and_then(|i| lookup(i).with_cache().with_size(16).find())
                    .map(|i| i.into()),
                exec: entry.exec().map(|c| c.into())?,
                working_dir: entry.path().and_then(|entry| entry.parse().ok()),
            })
        })
        .collect()
}
