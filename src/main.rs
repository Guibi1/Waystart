use std::{ops::Add, rc::Rc};

use gpui::{
    actions, div, img, prelude::FluentBuilder, px, size, uniform_list, AppContext, Application,
    Bounds, Context, Entity, Focusable, ImageSource, InteractiveElement, IntoElement, ObjectFit,
    ParentElement, Render, Resource, StatefulInteractiveElement, Styled, StyledImage,
    TitlebarOptions, Window, WindowBounds, WindowDecorations, WindowKind, WindowOptions,
};

use crate::components::{
    power_options::PowerOptions,
    ui::{separator, shortcut, TextInput, PALETTE},
};

mod components;
mod dapps;

actions!([Up, Down, Confirm]);

struct Waystart {
    desktop_entries: Vec<Rc<dapps::DesktopEntry>>,
    search_bar: Entity<TextInput>,
    power_options: Entity<PowerOptions>,
    selected: usize,
}

impl Waystart {
    fn new(dapps: Vec<dapps::DesktopEntry>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_bar = cx.new(|cx| TextInput::new(cx.focus_handle(), Some("Search")));
        window.focus(&search_bar.focus_handle(cx));

        Self {
            desktop_entries: dapps.into_iter().map(Rc::new).collect(),
            search_bar,
            power_options: cx.new(|_| PowerOptions::new()),
            selected: 0,
        }
    }
}

impl Render for Waystart {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_term = self.search_bar.read(cx).content().to_lowercase();
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
            .text_color(PALETTE.foreground)
            .bg(PALETTE.background)
            .border_color(PALETTE.border)
            .border_1()
            .overflow_hidden()
            .on_action(cx.listener(move |this, _: &Down, _, cx| {
                this.selected = this.selected.add(1).min(entries_count - 1);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &Up, _, cx| {
                this.selected = this.selected.saturating_sub(1);
                cx.notify();
            }))
            .on_action({
                let entry = entries.get(self.selected).cloned();
                move |_: &Confirm, _, _| {
                    if let Some(entry) = &entry {
                        entry.open()
                    }
                }
            })
            .child(self.search_bar.clone())
            .child(separator())
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .px_2()
                    .child(div().px_2().text_color(PALETTE.muted).child("Results"))
                    .child(
                        uniform_list(
                            "entry_list",
                            entries_count,
                            cx.processor(move |this, range: std::ops::Range<usize>, _, _| {
                                entries[range]
                                    .into_iter()
                                    .cloned()
                                    .enumerate()
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
                                            .hover(|style| style.bg(PALETTE.muted))
                                            .when(this.selected == i, |this| this.bg(PALETTE.muted))
                                            .when_some(entry.icon.as_ref(), |this, icon| {
                                                this.child(
                                                    img(ImageSource::Resource(Resource::Path(
                                                        icon.clone(),
                                                    )))
                                                    .size_8()
                                                    .object_fit(ObjectFit::Contain),
                                                )
                                            })
                                            .child(entry.name.clone())
                                            .when_some(
                                                entry.description.clone(),
                                                |this, description| {
                                                    this.child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(PALETTE.muted_foreground)
                                                            .child(description),
                                                    )
                                                },
                                            )
                                            .on_click(move |_e, _window, _app| entry.open())
                                    })
                                    .collect()
                            }),
                        )
                        .flex_grow()
                        .pb_2(),
                    ),
            )
            .child(separator())
            .child(
                div()
                    .px_2()
                    .py_1()
                    .flex()
                    .gap_2()
                    .child(self.power_options.clone())
                    .child(
                        div()
                            .ml_auto()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child("Open")
                            .child(shortcut("↵")),
                    ),
            )
    }
}

fn main() {
    let dapps = dapps::get_dapps();

    Application::new().run(|cx| {
        let bounds = Bounds::centered(None, size(px(800.), px(400.)), cx);
        components::ui::init(cx);
        // cx.bind_keys([
        //     KeyBinding::new("up", Up, None),
        //     KeyBinding::new("down", Down, None),
        //     KeyBinding::new("enter", Confirm, None),
        // ]);

        cx.open_window(
            WindowOptions {
                kind: WindowKind::PopUp,
                focus: true,
                // show: false,
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_decorations: Some(WindowDecorations::Client),
                titlebar: Some(TitlebarOptions {
                    title: Some("Waystart".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, app| app.new(|cx| Waystart::new(dapps, window, cx)),
        )
        .unwrap();

        cx.hide();
    });
}
