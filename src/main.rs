use gpui::{
    px, size, AppContext, Application, Bounds, Focusable, TitlebarOptions, WindowBounds,
    WindowDecorations, WindowKind, WindowOptions,
};

use crate::components::Waystart;

mod components;
mod desktop_entry;

fn main() {
    let desktop_entries = desktop_entry::get_desktop_entries();

    Application::new().run(|cx| {
        let bounds = Bounds::centered(None, size(px(800.), px(400.)), cx);
        components::init(cx);

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
            |window, cx| {
                let root = cx.new(|cx| Waystart::new(desktop_entries, cx));
                window.focus(&root.focus_handle(cx));
                root
            },
        )
        .unwrap();

        cx.hide();
    });
}
