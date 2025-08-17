use std::rc::Rc;

use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, ImageSource, InteractiveElement,
    IntoElement, KeyBinding, ObjectFit, ParentElement, Render, Resource,
    StatefulInteractiveElement, Styled, StyledImage, Window, actions, div, img, uniform_list,
};

use crate::config::Config;
use crate::desktop_entry;
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
    desktop_entries: Vec<Rc<desktop_entry::DesktopEntry>>,
    search_bar: Entity<TextInput>,
    selected: usize,
}

impl Waystart {
    pub fn new(desktop_entries: Vec<desktop_entry::DesktopEntry>, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            desktop_entries: desktop_entries.into_iter().map(Rc::new).collect(),
            search_bar: cx.new(|_| TextInput::new(focus_handle.clone()).placeholder("Search")),
            focus_handle,
            selected: 0,
        }
    }
}

impl Render for Waystart {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config = cx.global::<Config>();
        let search_term = self.search_bar.read(cx).value().to_lowercase();
        let entries = self
            .desktop_entries
            .iter()
            .filter(|entry| entry.name.to_lowercase().contains(&search_term))
            .cloned()
            .collect::<Vec<_>>();
        let entries_count = entries.len();

        if self.selected >= entries_count {
            self.selected = entries_count.saturating_sub(1);
        }

        div()
            .size_full()
            .flex()
            .flex_col()
            .text_color(config.foreground)
            .bg(config.background)
            .border_color(config.border)
            .border_1()
            .overflow_hidden()
            .track_focus(&self.focus_handle(cx))
            .key_context(CONTEXT)
            .on_action(cx.listener(move |this, _: &SelectPrev, _, cx| {
                this.selected = if this.selected == 0 {
                    entries_count.saturating_sub(1)
                } else {
                    this.selected.saturating_sub(1)
                };
                cx.notify();
            }))
            .on_action(cx.listener(move |this, _: &SelectNext, _, cx| {
                this.selected = if this.selected == entries_count.saturating_sub(1) {
                    0
                } else {
                    this.selected + 1
                };
                cx.notify();
            }))
            .on_action({
                let entry = entries.get(self.selected).cloned();
                move |_: &OpenProgram, _, cx| {
                    if let Some(entry) = &entry {
                        entry.open(cx)
                    }
                }
            })
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
                            entries_count,
                            cx.processor(move |this, range: std::ops::Range<usize>, _, cx| {
                                let config = cx.global::<Config>();
                                entries
                                    .iter()
                                    .cloned()
                                    .enumerate()
                                    .skip(range.start)
                                    .take(range.end - range.start)
                                    .map(|(i, entry)| {
                                        div()
                                            .id(entry.name.clone())
                                            .w_full()
                                            .px_4()
                                            .py_1p5()
                                            .flex()
                                            .items_center()
                                            .gap_4()
                                            .rounded_lg()
                                            .when_some(entry.icon.as_ref(), |this, icon| {
                                                this.child(
                                                    img(ImageSource::Resource(Resource::Path(
                                                        icon.clone(),
                                                    )))
                                                    .size_4()
                                                    .object_fit(ObjectFit::Contain),
                                                )
                                            })
                                            .child(entry.name.clone())
                                            .when(i == this.selected, |this| {
                                                this.bg(config.muted).when_some(
                                                    entry.description.clone(),
                                                    |this, description| {
                                                        this.child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(config.muted_foreground)
                                                                .child(description),
                                                        )
                                                    },
                                                )
                                            })
                                            .on_mouse_move(
                                                cx.listener(move |this, _, _, _| this.selected = i),
                                            )
                                            .on_click(move |_, _, cx| entry.open(cx))
                                    })
                                    .collect()
                            }),
                        )
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
