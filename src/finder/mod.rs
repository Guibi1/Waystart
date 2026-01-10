use std::rc::Rc;

use gpui::{App, Resource, SharedString};

pub mod desktop;

pub trait Finder {
    fn new() -> Self;

    /// Returns the entries when no search is performed.
    fn default_entries(&self) -> impl Iterator<Item = Rc<dyn Entry>>;

    /// Returns the entries that match the given pattern.
    fn filtered_entries(
        &self,
        matcher: &mut nucleo_matcher::Matcher,
        search_term: &str,
    ) -> impl Iterator<Item = Rc<dyn Entry>>;
}

pub trait Entry {
    /// Get a unique identifier for this entry.
    fn id(&self) -> &SharedString;

    /// Get the main text of this entry.
    fn text(&self) -> &SharedString;

    /// Get the subtle description of this entry.
    fn description(&self) -> Option<&SharedString>;

    /// Get the icon of this entry.
    fn icon(&self) -> Option<&Resource>;

    /// Get the text to use for searching this entry.
    fn haystack(&self) -> nucleo_matcher::Utf32Str<'_>;

    /// If this entry can be favorited.
    fn can_favorite(&self) -> bool;

    /// Execute this entry per user's request.
    fn execute(&self, cx: &mut App) -> EntryExecuteResult;
}

pub enum EntryExecuteResult {
    ExecuteFailed,
    CloseWindow,
}
