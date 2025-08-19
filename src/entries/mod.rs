use std::ops::Deref;
use std::rc::Rc;

use gpui::{Global, Resource, SharedString};

use crate::entries::desktop_entry::DesktopEntry;

mod desktop_entry;

#[derive(Clone)]
pub enum Entry {
    Application(Rc<DesktopEntry>),
}

impl Entry {
    pub fn name(&self) -> &SharedString {
        match self {
            Entry::Application(entry) => &entry.name,
        }
    }

    pub fn description(&self) -> Option<&SharedString> {
        match self {
            Entry::Application(entry) => entry.description.as_ref(),
        }
    }

    pub fn icon(&self) -> Option<&Resource> {
        match self {
            Entry::Application(entry) => entry.icon.as_ref(),
        }
    }

    pub fn open(&self) -> bool {
        match self {
            Entry::Application(entry) => entry.open(),
        }
    }
}

#[derive(Clone)]
pub struct SearchEntries(Vec<Entry>);

impl SearchEntries {
    pub fn load() -> Self {
        let entries = desktop_entry::get_desktop_entries();
        Self(
            entries
                .into_iter()
                .map(Rc::new)
                .map(Entry::Application)
                .collect(),
        )
    }

    pub fn filtered(&self, search_term: &str) -> Self {
        Self(
            self.0
                .iter()
                .filter(|entry| {
                    entry.name().to_lowercase().contains(search_term)
                        || entry
                            .description()
                            .is_some_and(|desc| desc.to_lowercase().contains(search_term))
                })
                .cloned()
                .collect(),
        )
    }
}

impl Global for SearchEntries {}
impl Deref for SearchEntries {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
