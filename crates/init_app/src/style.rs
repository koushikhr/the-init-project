use iced::{Background, Border, Color, Shadow, Vector, border::Radius, widget::container};

// --- PALETTE ---
pub const BG_DARK: Color = Color::from_rgb(0.05, 0.05, 0.08);
pub const BG_CARD: Color = Color::from_rgb(0.15, 0.15, 0.20); // Made lighter for visibility
pub const ACCENT: Color = Color::from_rgb(0.3, 0.4, 0.95);
pub const ACCENT_HOVER: Color = Color::from_rgb(0.4, 0.5, 1.0);
pub const SUCCESS: Color = Color::from_rgb(0.2, 0.8, 0.4);
pub const TEXT_PRIMARY: Color = Color::WHITE;
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.75); // Made brighter
pub const BORDER_COLOR: Color = Color::from_rgb(0.3, 0.3, 0.35);

// --- COMPONENT STYLES ---

pub fn welcome_modal(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_CARD)),
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: Radius::from(16.0),
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(0.0, 10.0),
            blur_radius: 20.0,
        },
        ..Default::default()
    }
}

pub fn primary_button(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    let base = iced::widget::button::Style {
        background: Some(Background::Color(ACCENT)),
        text_color: TEXT_PRIMARY,
        border: Border {
            radius: Radius::from(8.0),
            ..Default::default()
        },
        ..Default::default()
    };
    match status {
        iced::widget::button::Status::Hovered => iced::widget::button::Style {
            background: Some(Background::Color(ACCENT_HOVER)),
            ..base
        },
        _ => base,
    }
}

pub fn main_container(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_DARK)),
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

// FIX: New Button Styles for Cards (instead of Container styles)
pub fn card_btn_active(
    _theme: &iced::Theme,
    _status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: Some(Background::Color(Color::from_rgb(0.2, 0.25, 0.4))), // Distinct Blueish
        text_color: TEXT_PRIMARY,
        border: Border {
            color: ACCENT,
            width: 2.0,
            radius: Radius::from(12.0),
        },
        ..Default::default()
    }
}

pub fn card_btn_inactive(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    let base = iced::widget::button::Style {
        background: Some(Background::Color(BG_CARD)),
        text_color: TEXT_PRIMARY,
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: Radius::from(12.0),
        },
        ..Default::default()
    };

    match status {
        iced::widget::button::Status::Hovered => iced::widget::button::Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.25))),
            ..base
        },
        _ => base,
    }
}
