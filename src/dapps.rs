// use freedesktop_desktop_entry::{default_paths, get_languages_from_env, DesktopEntry, Iter};
// use freedesktop_icons::lookup;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use gpui::SharedString;

pub struct DesktopEntry {
    pub name: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<Arc<Path>>,
    pub exec: String,
    pub working_dir: Option<PathBuf>,
}

impl DesktopEntry {
    pub fn open(&self) {
        let mut cmd = Command::new(&self.exec);
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }

        if let Err(e) = cmd.spawn() {
            eprintln!("Failed to launch {}: {}", self.name, e);
        }
    }
}

pub fn get_dapps() -> Vec<DesktopEntry> {
    // let locales = get_languages_from_env();

    // Iter::new(default_paths())
    //     .flat_map(|path| DesktopEntry::from_path(path, Some(&locales)))
    //     .filter_map(|entry| {
    //         let exec = entry.exec();
    //         let name = entry.name(&locales);
    //         let icon = entry
    //             .icon()
    //             .map(|i| lookup(i).with_cache().with_size(64).find())
    //             .flatten();

    //         Some(DappEntry {
    //             name: name?.to_string(),
    //             icon: icon,
    //             exec: exec?.to_string(),
    //         })
    //     })
    //     .collect()
    vec![
        DesktopEntry {
            name: "Example Dapp".into(),
            description: Some("An example decentralized application".into()),
            icon: None, // Some(PathBuf::from("path/to/icon.png")),
            exec: "example_dapp_command".into(),
            working_dir: None,
        },
        DesktopEntry {
            name: "Another Dapp1".into(),
            description: None,
            icon: None, // Some(PathBuf::from("path/to/another_icon.png")),
            exec: "another_dapp_command".into(),
            working_dir: None,
        },
        DesktopEntry {
            name: "Another Dapp2".into(),
            description: None,
            icon: None, // Some(PathBuf::from("path/to/another_icon.png")),
            exec: "another_dapp_command".into(),
            working_dir: None,
        },
        DesktopEntry {
            name: "Another Dapp3".into(),
            description: None,
            icon: None, // Some(PathBuf::from("path/to/another_icon.png")),
            exec: "another_dapp_command".into(),
            working_dir: None,
        },
        DesktopEntry {
            name: "Another Dapp4".into(),
            description: None,
            icon: None, // Some(PathBuf::from("path/to/another_icon.png")),
            exec: "another_dapp_command".into(),
            working_dir: None,
        },
        DesktopEntry {
            name: "Another Dapp5".into(),
            description: None,
            icon: None, // Some(PathBuf::from("path/to/another_icon.png")),
            exec: "another_dapp_command".into(),
            working_dir: None,
        },
        DesktopEntry {
            name: "Another Dapp6".into(),
            description: None,
            icon: None, // Some(PathBuf::from("path/to/another_icon.png")),
            exec: "another_dapp_command".into(),
            working_dir: None,
        },
        DesktopEntry {
            name: "Another Dapp7".into(),
            description: None,
            icon: None, // Some(PathBuf::from("path/to/another_icon.png")),
            exec: "another_dapp_command".into(),
            working_dir: None,
        },
        DesktopEntry {
            name: "Another Dapp8".into(),
            description: None,
            icon: None, // Some(PathBuf::from("path/to/another_icon.png")),
            exec: "another_dapp_command".into(),
            working_dir: None,
        },
        DesktopEntry {
            name: "Another Dapp9".into(),
            description: None,
            icon: None, // Some(PathBuf::from("path/to/another_icon.png")),
            exec: "another_dapp_command".into(),
            working_dir: None,
        },
    ]
}
