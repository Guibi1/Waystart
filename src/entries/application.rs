use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use freedesktop_desktop_entry::{Iter, default_paths, get_languages_from_env};
use freedesktop_icons::lookup;
use gpui::{Resource, SharedString};
use nucleo_matcher::Utf32String;

use crate::config::Config;

#[derive(Debug)]
pub struct Application {
    pub id: SharedString,
    pub name: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<Resource>,
    pub searchable: Utf32String,
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

    pub fn load() -> Vec<Self> {
        let locales = get_languages_from_env();

        let mut applications = Vec::new();
        for entry in Iter::new(default_paths()).entries(Some(&locales)) {
            if entry.no_display() || entry.hidden() {
                continue;
            }

            let id = SharedString::from(entry.id().to_string());
            if applications.iter().any(|a: &Application| a.id == id) {
                continue;
            }

            let Ok(exec) = entry.parse_exec_with_uris(&[], &locales) else {
                continue;
            };
            let name = match entry.name(&locales) {
                Some(name) => SharedString::from(name.into_owned()),
                None => continue,
            };
            let description = entry
                .comment(&locales)
                .map(|description| SharedString::from(description.into_owned()));
            let icon = entry
                .icon()
                .and_then(|icon| lookup(icon).with_cache().with_size(28).find())
                .map(|path| Resource::Path(path.into()));
            let searchable = Utf32String::from(match description {
                Some(ref d) => name.to_string() + " " + d.as_str(),
                None => name.to_string(),
            });

            applications.push(Application {
                id,
                name,
                description,
                icon,
                searchable,
                exec,
                working_dir: entry.path().and_then(|entry| entry.parse().ok()),
                open_in_terminal: entry.terminal(),
            });
        }

        applications
    }
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
    .find(|term| paths.iter().any(|dir| dir.join(term).is_file()))
    .expect(
        "Failed to find a terminal emulator in your PATH. Please use the config to specify one.",
    )
});
