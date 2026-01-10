use gpui::{App, Resource, SharedString};

use crate::finder::{Entry, EntryExecuteResult};

pub struct MathEntry {
    pub result: SharedString,
}

impl Entry for MathEntry {
    fn id(&self) -> SharedString {
        self.result.clone()
    }

    fn text(&self) -> SharedString {
        self.result.clone()
    }

    fn description(&self) -> Option<SharedString> {
        None
    }

    fn icon(&self) -> Option<Resource> {
        None
    }

    fn can_favorite(&self) -> bool {
        false
    }

    fn execute(&self, cx: &mut App) -> EntryExecuteResult {
        cx.write_to_clipboard(self.result.to_string().into());
        EntryExecuteResult::CloseWindow
    }
}
