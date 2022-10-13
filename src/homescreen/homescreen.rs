use super::{icons::icon, HomeScreenState, Message};
use iced_native::{
    command::Command,
    program::Program,
    widget::{button, column::Column, row::Row, slider, text::Text, Container},
    Alignment, Color, Element, Length,
};
use iced_wgpu::Renderer;

pub fn HomeScreen<'a>(self1: &'a HomeScreenState) -> Container<'a, Message, Renderer> {
    Container::new(
        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Start)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Start)
                    .push(Column::new().padding(10).spacing(10).push(
                        Text::new(format!("Render at : {} fps", self1.fps.to_string())).size(28),
                    ))
                    .push(
                        button::Button::new(
                            Column::new()
                                .width(Length::Fill)
                                .align_items(Alignment::Start)
                                .push(Text::new("Press Here"))
                                .push(icon('\u{f004}', 20)),
                        )
                        .on_press(Message::UpdateHomeScreen(self1.clone())),
                    ),
            ),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}
