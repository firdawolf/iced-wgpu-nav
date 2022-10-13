use iced_native::{
    alignment::Horizontal,
    widget::{button, column::Column, row::Row, slider, text::Text, Container},
    Color, Element, Font, Length,
};
use iced_wgpu::Renderer;
const ICONS: Font = Font::External {
    name: "fa-regular-400",
    bytes: include_bytes!("../../fonts/fa-regular-400.ttf"),
};

pub fn icon(unicode: char, size: u16) -> Text<Renderer> {
    Text::new(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(size))
        .horizontal_alignment(Horizontal::Center)
        .size(size)
}
