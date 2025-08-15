use freedesktop_desktop_entry::{default_paths, get_languages_from_env, Iter};
use freedesktop_icons::lookup;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;

use gpui::{App, SharedString};

#[derive(Debug)]
pub struct DesktopEntry {
    pub name: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<Arc<Path>>,
    pub exec: Vec<String>,
    pub working_dir: Option<PathBuf>,
    pub open_in_terminal: bool,
}

impl DesktopEntry {
    pub fn open(&self, cx: &mut App) {
        let mut exec = self.exec.iter();

        let mut cmd = if self.open_in_terminal {
            let terminal = env::var_os("TERMINAL")
                .map(|s| {
                    s.into_string()
                        .expect("The $TERMINAL environment variable should be a valid UTF-8 string")
                })
                .unwrap_or_else(|| String::from("ghostty"));
            let mut cmd = Command::new(terminal);
            cmd.arg("-e");
            cmd
        } else {
            Command::new(exec.next().unwrap())
        };

        cmd.args(exec);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        if let Some(cwd) = &self.working_dir {
            cmd.current_dir(cwd);
        } else if let Some(cwd) = env::home_dir() {
            cmd.current_dir(cwd);
        }

        match cmd.spawn() {
            Ok(_) => cx.quit(),
            Err(e) => {
                eprintln!("Failed to launch {}: {}.", self.name, e);
                if self.open_in_terminal {
                    eprintln!("Please ensure that the $TERMINAL env is correct, or override it in the config.");
                }
            }
        }
    }
}

pub fn get_desktop_entries() -> Vec<DesktopEntry> {
    let locales = get_languages_from_env();

    Iter::new(default_paths())
        .flat_map(|path| freedesktop_desktop_entry::DesktopEntry::from_path(path, Some(&locales)))
        .filter_map(|entry| {
            if entry.no_display() || entry.hidden() {
                return None;
            }

            Some(DesktopEntry {
                name: entry.name(&locales).map(|c| c.into_owned().into())?,
                description: entry.comment(&locales).map(|c| c.into_owned().into()),
                icon: entry
                    .icon()
                    .and_then(|i| lookup(i).with_cache().with_size(16).find())
                    .map(|i| i.into()),
                exec: entry.parse_exec_with_uris(&[], &locales).ok()?,
                working_dir: entry.path().and_then(|entry| entry.parse().ok()),
                open_in_terminal: entry.terminal(),
            })
        })
        .collect()
}
