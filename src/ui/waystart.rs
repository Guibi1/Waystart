use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    KeyBinding, ParentElement, Render, ScrollStrategy, StatefulInteractiveElement, Styled,
    UniformListScrollHandle, Window, div, uniform_list,
};

use crate::config::Config;
use crate::finder::favorites::Favorites;
use crate::finder::{Entry, EntryExecuteResult, Finder, default_finders};
use crate::ui::actions::{Close, ExecuteEntry, SelectNextEntry, SelectPrevEntry, ToggleFavorite};
use crate::ui::elements::{EntryButton, Icon, PowerOptions, Separator, Shortcut, TextInput};

const CONTEXT: &str = "Waystart";

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", SelectPrevEntry, Some(CONTEXT)),
        KeyBinding::new("down", SelectNextEntry, Some(CONTEXT)),
        KeyBinding::new("shift-tab", SelectPrevEntry, Some(CONTEXT)),
        KeyBinding::new("tab", SelectNextEntry, Some(CONTEXT)),
        KeyBinding::new("enter", ExecuteEntry, Some(CONTEXT)),
        KeyBinding::new("secondary-d", ToggleFavorite, Some(CONTEXT)),
        KeyBinding::new("escape", Close, Some(CONTEXT)),
    ]);
}

pub struct Waystart {
    focus_handle: FocusHandle,
    entries: Vec<Rc<dyn Entry>>,
    list_scroll_handle: UniformListScrollHandle,
    search_bar: Entity<TextInput>,
    selected: usize,

    favorites: Favorites,
    finders: Vec<Box<dyn Finder>>,
    matcher: RefCell<nucleo_matcher::Matcher>,
}

impl Waystart {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let search_bar = cx.new(|_| {
            TextInput::new(focus_handle.clone()).placeholder("Search for apps and commands...")
        });

        cx.observe(&search_bar, |this, _, cx| this.filter_results(cx))
            .detach();
        cx.on_app_quit(|this, _| {
            this.favorites.save();
            async {}
        })
        .detach();

        let finders = default_finders();

        Self {
            focus_handle,
            entries: finders
                .iter()
                .filter_map(|finder| finder.default_entries())
                .flatten()
                .collect(),
            list_scroll_handle: UniformListScrollHandle::new(),
            search_bar,
            selected: 0,

            favorites: Favorites::load(),
            finders,
            matcher: RefCell::new(nucleo_matcher::Matcher::default()),
        }
    }

    pub fn reset_search(&mut self, cx: &mut Context<Self>) {
        self.search_bar
            .update(cx, |search_bar, _| search_bar.reset());
    }

    fn filter_results(&mut self, cx: &mut Context<Self>) {
        self.entries.clear();
        let search_term = self.search_bar.read(cx).content().trim();

        if search_term.is_empty() {
            self.entries.extend(
                self.finders
                    .iter()
                    .filter_map(|finder| finder.default_entries())
                    .flatten(),
            );
        } else {
            let mut matcher = self.matcher.borrow_mut();
            self.entries.extend(
                self.finders
                    .iter()
                    .filter_map(|finder| finder.filtered_entries(&mut matcher, search_term))
                    .flatten(),
            );
        }

        self.entries.sort_by_key(|entry| Reverse(entry.score()));
        self.selected = 0;
        cx.notify();
    }

    fn on_close(_: &Close, window: &mut Window, _cx: &mut App) {
        window.remove_window();
    }

    fn select_prev_entry<A>(&mut self, _: &A, _window: &mut Window, cx: &mut Context<Self>) {
        if self.selected == 0 {
            self.selected = self.entries.len().saturating_sub(1);
        } else {
            self.selected -= 1;
        };
        self.list_scroll_handle
            .scroll_to_item(self.selected, ScrollStrategy::Top);
        cx.notify();
    }

    fn select_next_entry<A>(&mut self, _: &A, _window: &mut Window, cx: &mut Context<Self>) {
        if self.selected + 1 == self.entries.len() {
            self.selected = 0;
        } else {
            self.selected += 1;
        };
        self.list_scroll_handle
            .scroll_to_item(self.selected, ScrollStrategy::Top);
        cx.notify();
    }

    fn execute_entry<A>(&mut self, _: &A, window: &mut Window, cx: &mut Context<Self>) {
        let entry = self.entries.get(self.selected).cloned();
        if let Some(ref entry) = entry
            && let EntryExecuteResult::CloseWindow = entry.execute(cx)
        {
            window.remove_window()
        }
    }

    fn toggle_favorite_entry<A>(&mut self, _: &A, _: &mut Window, _: &mut Context<Self>) {
        let entry = self.entries.get(self.selected).cloned();
        if let Some(ref entry) = entry
            && entry.can_favorite()
        {
            self.favorites.insert(entry.id().clone());
        }
    }
}

impl Render for Waystart {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config = cx.global::<Config>();
        let is_searching = !self.search_bar.read(cx).content().is_empty();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(config.theme.background)
            .text_color(config.theme.foreground)
            .font_family(config.theme.font_family.clone())
            .border_color(config.theme.border)
            .border_1()
            .rounded_lg()
            .overflow_hidden()
            .track_focus(&self.focus_handle(cx))
            .key_context(CONTEXT)
            .on_action::<Close>(Self::on_close)
            .on_action::<SelectPrevEntry>(cx.listener(Self::select_prev_entry))
            .on_action::<SelectNextEntry>(cx.listener(Self::select_next_entry))
            .on_action::<ExecuteEntry>(cx.listener(Self::execute_entry))
            .on_action::<ToggleFavorite>(cx.listener(Self::toggle_favorite_entry))
            .child(
                div()
                    .h_16()
                    .flex()
                    .pl_6()
                    .items_center()
                    .child(Icon::Search.build(config.theme.foreground))
                    .child(self.search_bar.clone()),
            )
            .child(Separator::new())
            .when(!is_searching, |this| {
                let mut favorites = self
                    .favorites
                    .iter()
                    .filter_map(|id| self.entries.iter().find(|entry| entry.id() == *id))
                    .map(|entry| EntryButton::new(entry.clone(), false).favorite(true))
                    .peekable();
                this.when(favorites.peek().is_some(), |this| {
                    this.child(
                        div()
                            .gap_1()
                            .px_2()
                            .child(
                                div()
                                    .px_5()
                                    .py_1()
                                    .text_color(config.theme.muted_foreground)
                                    .child("Favorites"),
                            )
                            .child(div().flex().gap_2().items_center().children(favorites)),
                    )
                })
            })
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .px_2()
                    .child(
                        div()
                            .px_5()
                            .py_1()
                            .text_color(config.theme.muted_foreground)
                            .child(if is_searching { "Results" } else { "Recents" }),
                    )
                    .child(
                        uniform_list(
                            "entry_list",
                            self.entries.len(),
                            cx.processor(move |this, range: std::ops::Range<usize>, _, cx| {
                                this.entries
                                    .iter()
                                    .enumerate()
                                    .skip(range.start)
                                    .take(range.end - range.start)
                                    .map(|(i, entry)| {
                                        div()
                                            .id(entry.id().clone())
                                            .child(EntryButton::new(
                                                entry.clone(),
                                                i == this.selected,
                                            ))
                                            .on_click(cx.listener(Self::execute_entry))
                                            .on_mouse_move(cx.listener(move |this, _, _, cx| {
                                                if i != this.selected {
                                                    this.selected = i;
                                                    cx.notify();
                                                }
                                            }))
                                    })
                                    .collect()
                            }),
                        )
                        .track_scroll(&self.list_scroll_handle)
                        .flex_grow()
                        .pb_2(),
                    ),
            )
            .child(Separator::new())
            .child(
                div()
                    .h_12()
                    .flex()
                    .px_4()
                    .items_center()
                    .gap_2()
                    .child(PowerOptions::new())
                    .child(
                        div()
                            .ml_auto()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child("Open")
                            .child(Shortcut::new("↵")),
                    ),
            )
    }
}

impl Focusable for Waystart {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
