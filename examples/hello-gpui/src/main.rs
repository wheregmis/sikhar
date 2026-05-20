use sparsha::prelude::*;
use sparsha::{element_button, element_component, ElementNode, ParentElement, Styled};
use sparsha::core::Color;

fn main() -> Result<(), sparsha::AppRunError> {
    #[cfg(target_arch = "wasm32")]
    sparsha::init_web()?;

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();

    App::builder()
        .title("Hello GPUI")
        .width(900)
        .height(700)
        .theme(Theme::dark())
        .router(
            Router::builder()
                .routes(vec![
                    Route::new("/", || {
                        element_component().render(hello_world_page).call()
                    }),
                    Route::new("/scroll", || {
                        element_component().render(scrollable_page).call()
                    }),
                ])
                .fallback("/")
                .build(),
        )
        .build()
        .run()
}

fn hello_world_page(cx: &mut ComponentContext<'_>) -> ElementNode {
    let count = cx.signal(0u32);
    let theme = cx.theme();

    div()
        .flex()
        .flex_col()
        .gap_3()
        .p_6()
        .size_full()
        .bg(theme.background_color())
        .child(
            div()
                .text_2xl()
                .font_bold()
                .text_color(theme.text_color())
                .child("Hello GPUI-style Sparsha"),
        )
        .child(
            div()
                .text_base()
                .text_color(theme.muted_text_color())
                .child("Element composition with div().flex().flex_col().gap_3()"),
        )
        .child(
            div()
                .flex()
                .gap_3()
                .child(
                    element_button("Increment")
                        .px_4()
                        .py_2()
                        .rounded_md()
                        .bg(theme.primary_color())
                        .on_click({
                            let count = count.clone();
                            move || count.set(count.get() + 1)
                        }),
                )
                .child(
                    div()
                        .text_lg()
                        .font_semibold()
                        .text_color(Color::WHITE)
                        .child(format!("Count: {}", count.get())),
                ),
        )
        .into_element()
}

fn scrollable_page(cx: &mut ComponentContext<'_>) -> ElementNode {
    let theme = cx.theme();
    let rows = (0..40)
        .map(|index| {
            div()
                .p_4()
                .rounded_md()
                .border_1()
                .border_color(theme.border_color())
                .bg(theme.surface_color())
                .child(format!("Scrollable row {}", index + 1))
        })
        .collect::<Vec<_>>();

    div()
        .flex()
        .flex_col()
        .size_full()
        .p_4()
        .gap_3()
        .bg(theme.background_color())
        .child(
            div()
                .text_xl()
                .font_bold()
                .text_color(theme.text_color())
                .child("Scrollable element surface"),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .overflow_y_scroll()
                .p_2()
                .rounded_lg()
                .border_1()
                .border_color(theme.border_color())
                .children(rows),
        )
        .into_element()
}
