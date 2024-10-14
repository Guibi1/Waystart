use gpui::*;

mod dapps;

struct Waystart {
    dapps: Vec<dapps::DappEntry>,
}

impl Render for Waystart {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let message_list = ListState::new(0, gpui::ListAlignment::Top, px(500.), move |ix, cx| {
            if let Some(dapp) = self.dapps.get(ix) {
                div()
                    .flex()
                    .flex_row()
                    .size_full()
                    .justify_start()
                    .items_center()
                    .child(
                        img(ImageSource::File(icon.into()))
                            .size(64)
                            .object_fit(gpui::ObjectFit::Contain)
                            .id(dapp.name + "_icon"),
                    )
                    .child(dapp.name.clone())
            }

            print!("Id {ix} is doenst exist in dapp list");
            div()
        });

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x2e7d32))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_3()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(list(message_list))
    }
}

fn main() {
    let dapps = dapps::get_dapps();

    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| Waystart { dapps }),
        )
        .unwrap();
    });
}
