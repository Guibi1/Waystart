use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, ImageSource, InteractiveElement,
    IntoElement, KeyBinding, ObjectFit, ParentElement, Render, ScrollStrategy,
    StatefulInteractiveElement, Styled, StyledImage, UniformListScrollHandle, Window, actions, div,
    img, uniform_list,
};

use crate::config::Config;
use crate::entries::SearchEntries;
use crate::ui::PowerOptions;
use crate::ui::elements::{Separator, Shortcut, TextInput};

actions!(waystart, [SelectPrev, SelectNext, OpenProgram, Close]);
const CONTEXT: &str = "Waystart";

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", SelectPrev, Some(CONTEXT)),
        KeyBinding::new("down", SelectNext, Some(CONTEXT)),
        KeyBinding::new("shift-tab", SelectPrev, Some(CONTEXT)),
        KeyBinding::new("tab", SelectNext, Some(CONTEXT)),
        KeyBinding::new("enter", OpenProgram, Some(CONTEXT)),
        KeyBinding::new("escape", Close, Some(CONTEXT)),
    ]);
}

pub struct Waystart {
    focus_handle: FocusHandle,
    entries: SearchEntries,
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
            entries: cx.global::<SearchEntries>().clone(),
            list_scroll_handle: UniformListScrollHandle::new(),
            search_bar,
            selected: 0,
        }
    }

    fn filter_results(&mut self, cx: &mut Context<Self>) {
        let search_term = self.search_bar.read(cx).content().to_lowercase();
        self.entries = cx.global::<SearchEntries>().filtered(&search_term);
        self.selected = 0;
        cx.notify();
    }
}

impl Render for Waystart {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config = cx.global::<Config>();

        div()
            .size_full()
            .flex()
            .flex_col()
            .text_color(config.foreground)
            .bg(config.background)
            .border_color(config.border)
            .border_1()
            .rounded_lg()
            .overflow_hidden()
            .track_focus(&self.focus_handle(cx))
            .key_context(CONTEXT)
            .on_action(cx.listener(move |this, _: &SelectPrev, _, cx| {
                if this.selected == 0 {
                    this.selected = this.entries.len().saturating_sub(1);
                } else {
                    this.selected = this.selected.saturating_sub(1);
                };
                this.list_scroll_handle
                    .scroll_to_item(this.selected, ScrollStrategy::Top);
                cx.notify();
            }))
            .on_action(cx.listener(move |this, _: &SelectNext, _, cx| {
                if this.selected + 1 == this.entries.len() {
                    this.selected = 0;
                } else {
                    this.selected += 1;
                };
                this.list_scroll_handle
                    .scroll_to_item(this.selected, ScrollStrategy::Top);
                cx.notify();
            }))
            .on_action(cx.listener(move |this, _: &OpenProgram, _, cx| {
                let entry = this.entries.get(this.selected).cloned();
                if let Some(entry) = &entry {
                    entry.open(cx)
                }
            }))
            .child(self.search_bar.clone())
            .child(Separator::new())
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .px_2()
                    .child(div().px_2().text_color(config.muted).child("Results"))
                    .child(
                        uniform_list(
                            "entry_list",
                            self.entries.len(),
                            cx.processor(move |this, range: std::ops::Range<usize>, _, cx| {
                                let config = cx.global::<Config>();

                                range
                                    .map(|i| {
                                        let entry = this.entries.get(i).unwrap().clone();

                                        div()
                                            .id(entry.name().clone())
                                            .w_full()
                                            .px_4()
                                            .py_1p5()
                                            .flex()
                                            .items_center()
                                            .gap_4()
                                            .rounded_lg()
                                            .when_some(entry.icon(), |this, icon| {
                                                this.child(
                                                    img(ImageSource::Resource(icon.clone()))
                                                        .size_4()
                                                        .object_fit(ObjectFit::Contain),
                                                )
                                            })
                                            .child(entry.name().clone())
                                            .when(i == this.selected, |this| {
                                                this.bg(config.muted).when_some(
                                                    entry.description(),
                                                    |this, description| {
                                                        this.child(
                                                            div()
                                                                .text_color(config.muted_foreground)
                                                                .child(description.clone()),
                                                        )
                                                    },
                                                )
                                            })
                                            .on_mouse_move(cx.listener(move |this, _, _, cx| {
                                                if this.selected != i {
                                                    this.selected = i;
                                                    cx.notify();
                                                }
                                            }))
                                            .on_click(move |_, _, cx| entry.open(cx))
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
                    .px_2()
                    .py_1()
                    .flex()
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
