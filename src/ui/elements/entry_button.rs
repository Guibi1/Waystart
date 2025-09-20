use gpui::{
    App, ImageSource, InteractiveElement, IntoElement, ObjectFit, ParentElement, RenderOnce,
    StatefulInteractiveElement, Styled, StyledImage, TextOverflow, Window, div, img,
    prelude::FluentBuilder,
};

use crate::{config::Config, entries::Entry};

#[derive(IntoElement)]
pub struct EntryButton {
    entry: Entry,
    selected: bool,
    favorite: bool,
    select: Box<dyn Fn(&mut App)>,
}

impl EntryButton {
    pub fn new(entry: Entry, selected: bool, select: impl Fn(&mut App) + 'static) -> EntryButton {
        EntryButton {
            entry,
            selected,
            favorite: false,
            select: Box::new(select),
        }
    }

    pub fn favorite(mut self, favorite: bool) -> Self {
        self.favorite = favorite;
        self
    }
}

impl RenderOnce for EntryButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let config = cx.global::<Config>();

        div()
            .id(self.entry.id().clone())
            .flex()
            .items_center()
            .rounded_lg()
            .when_else(
                self.favorite,
                |this| {
                    this.w_32()
                        .h_24()
                        .flex_col()
                        .justify_center()
                        .text_center()
                        .text_overflow(TextOverflow::Truncate("...".into()))
                        .hover(|this| this.bg(config.theme.muted))
                },
                |this| this.w_full().px_4().h_12(),
            )
            .when(self.selected, |this| this.bg(config.theme.muted))
            .when_some(self.entry.icon(), |this, icon| {
                this.child(
                    img(ImageSource::Resource(icon.clone()))
                        .size_7()
                        .mr_4()
                        .object_fit(ObjectFit::Contain),
                )
            })
            .child(self.entry.name().clone())
            .when_some(
                self.selected.then_some(self.entry.description()).flatten(),
                |this, description| {
                    this.child(
                        div()
                            .flex()
                            .text_color(config.theme.muted_foreground)
                            .when(self.selected, |this| this.bg(config.theme.muted))
                            .text_overflow(TextOverflow::Truncate("...".into()))
                            .when(!self.favorite, |this| this.child(" — "))
                            .child(description.clone()),
                    )
                },
            )
            .on_mouse_move(move |_, _, cx| {
                if !self.selected {
                    (self.select)(cx);
                }
            })
            .on_click(move |_, window, cx| {
                if self.entry.open(cx) {
                    window.remove_window();
                }
            })
    }
}
