use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::rc::Rc;

use freedesktop_icons::lookup;
use gpui::{App, Resource, SharedString};
use nucleo_matcher::Utf32String;

use crate::config::Config;
use crate::finder::desktop::{SearchEntries, create_terminal_command};
use crate::finder::{Entry, EntryExecuteResult};

pub struct DesktopEntry {
    pub id: SharedString,
    pub name: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<Resource>,
    pub haystack: nucleo_matcher::Utf32String,
    exec: Vec<String>,
    working_dir: Option<PathBuf>,
    open_in_terminal: bool,
}

impl Entry for DesktopEntry {
    fn id(&self) -> &SharedString {
        &self.id
    }

    fn text(&self) -> &SharedString {
        &self.name
    }

    fn description(&self) -> Option<&SharedString> {
        self.description.as_ref()
    }

    fn icon(&self) -> Option<&Resource> {
        self.icon.as_ref()
    }

    fn haystack(&self) -> nucleo_matcher::Utf32Str<'_> {
        self.haystack.slice(..)
    }

    fn can_favorite(&self) -> bool {
        true
    }

    fn execute(&self, cx: &mut App) -> EntryExecuteResult {
        cx.global_mut::<SearchEntries>()
            .increment_frequency(self.id());

        let config = cx.global::<Config>();
        let mut cmd = if self.open_in_terminal {
            create_terminal_command(config, &self.exec)
        } else {
            let [exec, args @ ..] = self.exec.as_slice() else {
                eprintln!("Failed to launch {}: Exec command was empty.", self.name);
                return EntryExecuteResult::ExecuteFailed;
            };

            let mut cmd = Command::new(exec);
            cmd.args(args);
            cmd
        };

        if let Some(ref cwd) = self.working_dir {
            cmd.current_dir(cwd);
        } else if let Some(cwd) = std::env::home_dir() {
            cmd.current_dir(cwd);
        }

        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        match cmd.spawn() {
            Ok(_) => EntryExecuteResult::CloseWindow,
            Err(e) => {
                eprintln!("Failed to launch {}: {}.", self.name, e);
                EntryExecuteResult::ExecuteFailed
            }
        }
    }
}

impl DesktopEntry {
    pub fn load() -> Vec<Rc<DesktopEntry>> {
        let paths = freedesktop_desktop_entry::default_paths();
        let locales = freedesktop_desktop_entry::get_languages_from_env();

        let mut entries = HashMap::<SharedString, DesktopEntry>::new();
        for entry in freedesktop_desktop_entry::Iter::new(paths).entries(Some(&locales)) {
            if entry.no_display() || entry.hidden() {
                continue;
            }

            let id = SharedString::from(entry.id().to_string());
            if entries.contains_key(&id) {
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
            let haystack = Utf32String::from(match description {
                Some(ref d) => name.to_string() + " " + d.as_str(),
                None => name.to_string(),
            });

            entries.insert(
                id.clone(),
                DesktopEntry {
                    id,
                    name,
                    description,
                    icon,
                    haystack,
                    exec,
                    working_dir: entry.path().and_then(|entry| entry.parse().ok()),
                    open_in_terminal: entry.terminal(),
                },
            );
        }

        entries.into_values().map(Rc::new).collect()
    }
}
