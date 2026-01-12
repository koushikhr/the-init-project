use crate::style;
use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, image as app_image, svg, text},
};
use init_core::manifest::App;

pub fn view<'a, Message: 'a + Clone>(
    app: &'a App,
    is_selected: bool,
    is_available: bool,
    on_toggle: impl Fn(String, bool) -> Message,
) -> Element<'a, Message> {
    // 1. Path Logic
    // Resolve relative to executable, fallback to current dir
    let base_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    let raw_path = app.icon.clone().unwrap_or("icons/default.svg".to_string());
    let abs_path = base_dir.join(&raw_path);
    let path_str = abs_path.to_string_lossy().to_string();

    // 2. Icon Widget (Fixed Size + ContentFit)
    let icon: Element<Message> = if raw_path.ends_with(".svg") {
        svg(path_str)
            .width(Length::Fixed(48.0))
            .height(Length::Fixed(48.0))
            .content_fit(iced::ContentFit::Contain)
            .into()
    } else {
        app_image(path_str)
            .width(Length::Fixed(48.0))
            .height(Length::Fixed(48.0))
            .content_fit(iced::ContentFit::Contain)
            .into()
    };

    let title = text(&app.name).size(15).color(style::TEXT_PRIMARY);

    let status = if is_selected {
        text("Selected").size(12).color(style::SUCCESS)
    } else if !is_available {
        text("Unavailable").size(12).color(style::TEXT_SECONDARY)
    } else {
        text("Click to Add").size(12).color(style::TEXT_SECONDARY)
    };

    let content = column![
        container(icon).height(60).center_y(Length::Fill),
        title,
        status
    ]
    .spacing(8)
    .align_x(Alignment::Center);

    // 3. Button Container
    let mut btn = button(
        container(content)
            .width(Length::Fixed(160.0))
            .height(Length::Fixed(180.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .padding(0)
    .style(if is_selected {
        style::card_btn_active
    } else {
        style::card_btn_inactive
    });

    if is_available {
        btn = btn.on_press(on_toggle(app.id.clone(), !is_selected));
    }

    btn.into()
}
