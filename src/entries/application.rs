use freedesktop_desktop_entry::{DesktopEntry, Iter, default_paths, get_languages_from_env};
use freedesktop_icons::lookup;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use gpui::{Resource, SharedString};

#[derive(Debug)]
pub struct Application {
    pub id: String,
    pub name: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<Resource>,
    exec: Vec<String>,
    working_dir: Option<PathBuf>,
    open_in_terminal: bool,
}

impl Application {
    pub fn open(&self) -> bool {
        let [exec, args @ ..] = self.exec.as_slice() else {
            eprintln!("Exec command was empty.");
            return false;
        };

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
            Command::new(exec)
        };

        cmd.args(args);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        if let Some(cwd) = &self.working_dir {
            cmd.current_dir(cwd);
        } else if let Some(cwd) = env::home_dir() {
            cmd.current_dir(cwd);
        }

        match cmd.spawn() {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Failed to launch {}: {}.", self.name, e);
                if self.open_in_terminal {
                    eprintln!(
                        "Please ensure that the $TERMINAL env is correct, or override it in the config."
                    );
                }
                false
            }
        }
    }
}

pub fn get_desktop_entries() -> Vec<Application> {
    let locales = get_languages_from_env();

    Iter::new(default_paths())
        .flat_map(|path| DesktopEntry::from_path(path, Some(&locales)))
        .filter_map(|entry| {
            if entry.no_display() || entry.hidden() {
                return None;
            }

            Some(Application {
                id: entry.id().to_string(),
                name: entry.name(&locales).map(|c| c.into_owned().into())?,
                description: entry.comment(&locales).map(|c| c.into_owned().into()),
                icon: entry
                    .icon()
                    .and_then(|i| lookup(i).with_cache().with_size(28).find())
                    .map(|i| i.into()),
                exec: entry.parse_exec_with_uris(&[], &locales).ok()?,
                working_dir: entry.path().and_then(|entry| entry.parse().ok()),
                open_in_terminal: entry.terminal(),
            })
        })
        .collect()
}
