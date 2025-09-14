use std::collections::HashSet;
use std::env;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use freedesktop_desktop_entry::{Iter, default_paths, get_languages_from_env};
use freedesktop_icons::lookup;
use gpui::{Resource, SharedString};

use crate::config::Config;

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
    pub fn open(&self, config: &Config) -> bool {
        let mut cmd = if self.open_in_terminal {
            let mut cmd = Command::new(config.terminal.as_deref().unwrap_or_else(|| *TERMINAL));
            cmd.arg("-e").args(&self.exec);
            cmd
        } else {
            let [exec, args @ ..] = self.exec.as_slice() else {
                eprintln!("Failed to launch {}: Exec command was empty.", self.name);
                return false;
            };

            let mut cmd = Command::new(exec);
            cmd.args(args);
            cmd
        };

        if let Some(ref cwd) = self.working_dir {
            cmd.current_dir(cwd);
        } else if let Some(cwd) = env::home_dir() {
            cmd.current_dir(cwd);
        }

        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        match cmd.spawn() {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Failed to launch {}: {}.", self.name, e);
                false
            }
        }
    }
}

impl Hash for Application {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Application {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Application {}

pub fn load_applications() -> Vec<Application> {
    let locales = get_languages_from_env();

    Iter::new(default_paths())
        .entries(Some(&locales))
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
        .collect::<HashSet<Application>>()
        .into_iter()
        .collect()
}

static TERMINAL: LazyLock<&str> = LazyLock::new(|| {
    let paths = env::split_paths(&env::var_os("PATH").unwrap()).collect::<Vec<_>>();
    [
        "ghostty",
        "kitty",
        "alacritty",
        "foot",
        "gnome-terminal",
        "konsole",
    ]
    .into_iter()
    .find(|term| paths.iter().any(|dir| dir.join(&term).is_file()))
    .expect(
        "Failed to find a terminal emulator in your PATH. Please use the config to specify one.",
    )
});
