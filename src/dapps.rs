use std::path::PathBuf;
use freedesktop_icons::lookup;
use freedesktop_desktop_entry::{ default_paths, get_languages_from_env, DesktopEntry, Iter };


pub struct DappEntry {
    pub name: String,
    pub icon: Option<PathBuf>,
    pub exec: String,
}

pub fn print_dapps() -> Vec<DappEntry> {
    let locales = get_languages_from_env();

    Iter::new(default_paths())
        .flat_map(|path| DesktopEntry::from_path(path, Some(&locales)))
        .filter_map(|entry| {
            let exec = entry.exec();
            let name = entry.name(&locales);
            let icon = entry.icon().map(|i| lookup(i).with_cache().find()).flatten();

            Some(DappEntry {
                name: name?.to_string(),
                icon: icon,
                exec: exec?.to_string(),
            })
        }).collect()
}
