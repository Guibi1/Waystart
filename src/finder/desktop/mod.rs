use std::cmp::Reverse;
use std::ops::Deref;
use std::rc::Rc;

use gpui::{Global, SharedString};
use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};

use crate::finder::desktop::entry::DesktopEntry;
use crate::finder::desktop::favorites::Favorites;
use crate::finder::desktop::frequency::{EntryFrequency, Frequencies};
use crate::finder::desktop::terminal::create_terminal_command;
use crate::finder::{Entry, Finder};

mod entry;
mod favorites;
mod frequency;
mod terminal;

pub struct SearchEntries {
    entries: Vec<Rc<DesktopEntry>>,
    frequencies: Frequencies,
    favorites: Favorites,
}

impl SearchEntries {
    pub async fn save(&self) {
        self.frequencies.save().await;
        self.favorites.save().await;
    }

    pub fn sort_by_frequency(&mut self) {
        self.entries.sort_by_cached_key(|e| {
            Reverse(
                self.frequencies
                    .get(e.id().as_str())
                    .cloned()
                    .unwrap_or_default(),
            )
        });
    }

    pub fn favorites(&self) -> Vec<Rc<dyn Entry>> {
        self.favorites
            .iter()
            .filter_map(|id| self.entries.iter().find(|entry| entry.id == *id))
            .map(|e| e.clone() as Rc<dyn Entry>)
            .collect()
    }

    pub fn add_favorite(&mut self, entry: &Rc<dyn Entry>) {
        self.favorites.insert(entry.id().clone());
    }

    pub fn increment_frequency(&mut self, entry_id: &SharedString) {
        if let Some(frequency) = self.frequencies.get_mut(entry_id) {
            frequency.increment();
        } else {
            self.frequencies
                .insert(entry_id.clone(), EntryFrequency::new());
        }
    }
}

impl Finder for SearchEntries {
    fn new() -> Self {
        Self {
            entries: DesktopEntry::load(),
            frequencies: Frequencies::load(),
            favorites: Favorites::load(),
        }
    }

    fn default_entries(&self) -> Option<Vec<Rc<dyn Entry>>> {
        Some(
            self.entries
                .iter()
                .map(|e| e.clone() as Rc<dyn Entry>)
                .collect(),
        )
    }

    fn filtered_entries(
        &self,
        matcher: &mut nucleo_matcher::Matcher,
        search_term: &str,
    ) -> Option<Vec<Rc<dyn Entry>>> {
        let search_pattern = Pattern::new(
            search_term,
            CaseMatching::Ignore,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );

        let mut result: Vec<_> = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(idx, entry)| {
                search_pattern
                    .score(entry.haystack.slice(..), matcher)
                    .map(|score| (idx, score))
            })
            .collect();
        result.sort_by_key(|(_, score)| Reverse(*score));

        Some(
            result
                .into_iter()
                .map(|(i, _)| self.entries[i].clone() as Rc<dyn Entry>)
                .collect(),
        )
    }
}

impl Global for SearchEntries {}
impl Deref for SearchEntries {
    type Target = Vec<Rc<DesktopEntry>>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}
