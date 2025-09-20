use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    KeyBinding, ParentElement, Render, ScrollStrategy, Styled, UniformListScrollHandle, Window,
    actions, div, uniform_list,
};

use crate::config::Config;
use crate::entries::{Entry, SearchEntries};
use crate::ui::PowerOptions;
use crate::ui::elements::{EntryButton, Icon, Separator, Shortcut, TextInput};

actions!(
    waystart,
    [SelectPrev, SelectNext, OpenProgram, AddFavorite, Close]
);
const CONTEXT: &str = "Waystart";

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", SelectPrev, Some(CONTEXT)),
        KeyBinding::new("down", SelectNext, Some(CONTEXT)),
        KeyBinding::new("shift-tab", SelectPrev, Some(CONTEXT)),
        KeyBinding::new("tab", SelectNext, Some(CONTEXT)),
        KeyBinding::new("enter", OpenProgram, Some(CONTEXT)),
        KeyBinding::new("secondary-d", AddFavorite, Some(CONTEXT)),
        KeyBinding::new("escape", Close, Some(CONTEXT)),
    ]);
}

pub struct Waystart {
    focus_handle: FocusHandle,
    entries: Vec<Entry>,
    list_scroll_handle: UniformListScrollHandle,
    search_bar: Entity<TextInput>,
    selected: usize,
}

impl Waystart {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let search_bar = cx.new(|_| {
            TextInput::new(focus_handle.clone()).placeholder("Search for apps and commands...")
        });

        cx.observe(&search_bar, |this, _, cx| this.filter_results(cx))
            .detach();
        cx.observe_global::<SearchEntries>(|this, cx| this.filter_results(cx))
            .detach();

        Self {
            focus_handle,
            entries: cx.global::<SearchEntries>().filtered(""),
            list_scroll_handle: UniformListScrollHandle::new(),
            search_bar,
            selected: 0,
        }
    }

    pub fn reset_search(&mut self, cx: &mut Context<Self>) {
        self.search_bar
            .update(cx, |search_bar, _| search_bar.reset());
        self.filter_results(cx);
    }

    fn filter_results(&mut self, cx: &mut Context<Self>) {
        if let Some(entries) = cx.try_global::<SearchEntries>() {
            let search_term = self.search_bar.read(cx).content().to_lowercase();
            self.entries = entries.filtered(&search_term);
            self.selected = 0;
            cx.notify();
        }
    }
}

impl Render for Waystart {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config = cx.global::<Config>();
        let favorites = cx.global::<SearchEntries>().favorites();
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
            .on_action::<Close>(|_, window, _| window.remove_window())
            .on_action::<SelectPrev>(cx.listener(move |this, _, _, cx| {
                if this.selected == 0 {
                    this.selected = this.entries.len().saturating_sub(1);
                } else {
                    this.selected = this.selected.saturating_sub(1);
                };
                this.list_scroll_handle
                    .scroll_to_item(this.selected, ScrollStrategy::Top);
                cx.notify();
            }))
            .on_action::<SelectNext>(cx.listener(move |this, _, _, cx| {
                if this.selected + 1 == this.entries.len() {
                    this.selected = 0;
                } else {
                    this.selected += 1;
                };
                this.list_scroll_handle
                    .scroll_to_item(this.selected, ScrollStrategy::Top);
                cx.notify();
            }))
            .on_action::<OpenProgram>(cx.listener(move |this, _, window, cx| {
                let entry = this.entries.get(this.selected).cloned();
                if let Some(ref entry) = entry
                    && entry.open(cx)
                {
                    window.remove_window();
                }
            }))
            .on_action::<AddFavorite>(cx.listener(move |this, _, _, cx| {
                let entry = this.entries.get(this.selected).cloned();
                if let Some(ref entry) = entry {
                    cx.global_mut::<SearchEntries>().add_favorite(entry);
                }
            }))
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
            .when(!favorites.is_empty() && !is_searching, |this| {
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
                        .child(
                            div().flex().gap_2().items_center().children(
                                favorites.into_iter().map(|entry| {
                                    EntryButton::new(entry, false, |_| {}).favorite(true)
                                }),
                            ),
                        ),
                )
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
                                range
                                    .map(|i| {
                                        let entity = cx.entity().downgrade();
                                        EntryButton::new(
                                            this.entries.get(i).unwrap().clone(),
                                            i == this.selected,
                                            move |cx| {
                                                entity
                                                    .update(cx, |this, cx| {
                                                        this.selected = i;
                                                        cx.notify();
                                                    })
                                                    .ok();
                                            },
                                        )
                                    })
                                    .collect()
                            }),
                        )
                        .track_scroll(self.list_scroll_handle.clone())
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
