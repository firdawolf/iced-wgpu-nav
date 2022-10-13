use iced_native::widget::{button, container, Container};

use super::nav_state::Message;

pub enum Button {
    FilterActive,
    FilterSelected,
    Special,
    Icon,
    Destructive,
}

pub enum Container {
    current,
    navContainer,
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::StyleSheet::Style {
        match self {
            Container::current => container::StyleSheet::Style {
                background: Some(Background::Color(Color::from_rgba8(1, 1, 1, 0.7))),
                ..container::Style::default()
            },
            Container::navContainer => container::Style {
                background: Some(Background::Color(Color::from_rgba8(1, 1, 1, 1.0))),
                ..container::Style::default()
            },
        }
    }
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::FilterActive => button::Style {
                background: Some(Background::Color(Color::from_rgba8(1, 1, 1, 1.0))),
                text_color: Color::from_rgba8(202, 204, 206, 1.0),

                ..button::Style::default()
            },
            Button::FilterSelected => button::Style {
                background: Some(Background::Color(Color::from_rgba8(105, 201, 208, 1.0))),
                border_radius: 6.0,

                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Special => button::Style {
                background: Some(Background::Color(Color::from_rgba8(238, 29, 82, 1.0))),
                border_radius: 4.0,
                text_color: Color::from_rgba8(202, 204, 206, 1.0),
                ..button::Style::default()
            },
            Button::Icon => button::Style {
                text_color: Color::from_rgb(0.5, 0.5, 0.5),
                ..button::Style::default()
            },
            Button::Destructive => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                border_radius: 5.0,
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 1.0),
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            text_color: match self {
                Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
                Button::FilterActive => Color::from_rgb(0.2, 0.2, 0.7),
                _ => active.text_color,
            },
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }
}
