use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_2()
            .child("Hello World!")
            .child(
                Button::new("hello")
                    .primary()
                    .label("Click me")
                    .on_click(|_, _, _| println!("Clicked!"))
            )
    }
}

fn main() {
    application()
        .with_assets(assets::Assets)
        .run(|cx| { 
            init(cx);

            let bounds = Bounds::centered(None, size(px(1024.), px(768.)), cx);

            open_window(WindowOptions{
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            }, cx, |window,cx| {
                window.set_window_title("Classic MS EXP Tracker");
                cx.new(|_| HelloWorld)
            })
            .expect("Failed to open window");
        })
}