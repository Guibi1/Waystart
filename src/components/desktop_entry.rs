use std::rc::Rc;

use gpui::prelude::FluentBuilder;
use gpui::{
    div, img, App, ImageSource, InteractiveElement, IntoElement, ObjectFit, ParentElement,
    RenderOnce, Resource, StatefulInteractiveElement, Styled, StyledImage, Window,
};

use crate::components::ui::palette::PALETTE;
use crate::desktop_entry;

#[derive(IntoElement)]
pub struct DesktopEntry {
    entry: Rc<desktop_entry::DesktopEntry>,
    selected: bool,
}

impl DesktopEntry {
    pub fn new(entry: Rc<desktop_entry::DesktopEntry>, selected: bool) -> Self {
        DesktopEntry { entry, selected }
    }
}

impl RenderOnce for DesktopEntry {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.entry.name.clone())
            .w_full()
            .px_4()
            .py_1p5()
            .flex()
            .items_center()
            .gap_4()
            .rounded_lg()
            .when_some(self.entry.icon.as_ref(), |this, icon| {
                this.child(
                    img(ImageSource::Resource(Resource::Path(icon.clone())))
                        .size_4()
                        .object_fit(ObjectFit::Contain),
                )
            })
            .child(self.entry.name.clone())
            .when(self.selected, |this| {
                this.bg(PALETTE.muted).when_some(
                    self.entry.description.clone(),
                    |this, description| {
                        this.child(
                            div()
                                .text_sm()
                                .text_color(PALETTE.muted_foreground)
                                .child(description),
                        )
                    },
                )
            })
            .on_click(move |_, _, cx| self.entry.open(cx))
    }
}
