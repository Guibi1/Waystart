use std::cmp::Reverse;
use std::ops::Deref;
use std::rc::Rc;

use gpui::{App, Global, Resource, SharedString};

use crate::config::Config;
use crate::entries::application::Application;
use crate::entries::frequency::{EntryFrequency, Frequencies};

mod application;
mod frequency;

#[derive(Clone)]
pub enum Entry {
    Application(Rc<Application>),
}

impl Entry {
    pub fn id(&self) -> &str {
        match self {
            Entry::Application(entry) => &entry.id,
        }
    }

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

    pub fn open(&self, cx: &mut App) -> bool {
        cx.global_mut::<SearchEntries>()
            .increment_frequency(self.id());

        let config = cx.global::<Config>();
        match self {
            Entry::Application(entry) => entry.open(config),
        }
    }
}

pub struct SearchEntries {
    entries: Vec<Entry>,
    frequencies: Frequencies,
}

impl SearchEntries {
    pub fn load() -> Self {
        let entries = application::load_applications()
            .into_iter()
            .map(Rc::new)
            .map(Entry::Application)
            .collect();

        Self {
            entries,
            frequencies: Frequencies::load(),
        }
    }

    pub async fn save(&self) {
        self.frequencies.save().await;
    }

    pub fn sort_by_frequency(&mut self) {
        self.entries.sort_by_cached_key(|e| {
            Reverse(self.frequencies.get(e.id()).cloned().unwrap_or_default())
        });
    }

    pub fn filtered(&self, search_term: &str) -> Vec<Entry> {
        if search_term.trim().is_empty() {
            return self.entries.clone();
        }

        self.entries
            .iter()
            .filter(|entry| {
                entry.name().to_lowercase().contains(search_term)
                    || entry
                        .description()
                        .is_some_and(|desc| desc.to_lowercase().contains(search_term))
            })
            .cloned()
            .collect()
    }

    pub fn increment_frequency(&mut self, entry_id: &str) {
        if let Some(frequency) = self.frequencies.get_mut(entry_id) {
            frequency.increment();
        } else {
            self.frequencies
                .insert(entry_id.to_string(), EntryFrequency::new());
        }
    }
}

impl Global for SearchEntries {}
impl Deref for SearchEntries {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}
