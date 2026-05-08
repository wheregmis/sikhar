use sparsha::prelude::*;

fn material_counter_theme() -> Theme {
    Theme::light()
        .background(Color::from_hex(0xFAFAFA))
        .surface(Color::WHITE)
        .surface_variant(Color::from_hex(0xF5F5F5))
        .text(Color::from_hex(0x212121))
        .muted_text(Color::from_hex(0x757575))
        .brand_states(
            Color::from_hex(0x2196F3),
            Color::from_hex(0x1E88E5),
            Color::from_hex(0x1976D2),
        )
        .border(Color::from_hex(0xE0E0E0))
        .type_scale(16.0, 14.0, 20.0, 16.0)
        .radius(4.0, 6.0, 28.0)
        .control_size(40.0)
        .control_padding(16.0, 10.0)
}

fn main() -> Result<(), sparsha::AppRunError> {
    #[cfg(target_arch = "wasm32")]
    sparsha::init_web()?;

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();

    App::builder()
        .title("Sparsha Counter")
        .width(430)
        .height(760)
        .theme(material_counter_theme())
        .router(
            Router::builder()
                .routes(vec![Route::new("/", || {
                    component().render(counter_app).call()
                })])
                .fallback("/")
                .build(),
        )
        .build()
        .run()
}

fn counter_app(cx: &mut ComponentContext<'_>) -> Scaffold {
    let count = cx.signal(0i32);
    let theme = cx.theme();

    Scaffold::new(Center::new(Padding::all(
        24.0,
        Container::column()
            .fill_width()
            .gap(theme.spacing_md())
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .child(
                Text::builder()
                    .content("You have pushed the button this many times:")
                    .font_size(16.0)
                    .color(theme.muted_text_color())
                    .fill_width(true)
                    .align(TextAlign::Center)
                    .overflow(TextOverflow::Clip)
                    .build(),
            )
            .child(
                Text::builder()
                    .content(count.get().to_string())
                    .font_size(72.0)
                    .bold(true)
                    .color(theme.text_color())
                    .fill_width(true)
                    .align(TextAlign::Center)
                    .overflow(TextOverflow::Clip)
                    .build(),
            ),
    )))
    .background(theme.background_color())
    .app_bar(AppBar::new("Sparsha Demo Home Page").center_title(true))
    .floating_action_button(
        FloatingActionButton::new("+")
            .accessibility_label("Increment counter")
            .on_click(move || {
                count.set(count.get() + 1);
            }),
    )
}
