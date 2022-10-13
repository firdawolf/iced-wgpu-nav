use super::public_state::nav_state::{Message, PublicScreenState};

use super::icons::icon;
use iced_native::alignment;
use iced_native::widget::{Button, Column, Container, Row, Text};
use iced_native::Length;

#[path = "../public_state/mod.rs"]
mod public_state;
use iced_wgpu::Renderer;

pub fn navbar<'a>(self1: &'a PublicScreenState) -> Container<'a, Message, Renderer> {
    let content1 = |text1: String, icon_code: char| -> Container<Message, Renderer> {
        return Container::new(
            Column::new()
                .align_items(alignment::Alignment::Center)
                .push(icon(icon_code, 20))
                .push(Text::new(text1).size(10)),
        )
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill);
    };
    return Container::new(
        Row::new()
            .push(
                Container::new(
                    Button::new(content1(String::from("Home"), '\u{f015}'))
                        // .style(if self1 == 0 {
                        // } else {
                        //     nav_style::buttonStyle::FilterActive
                        // })
                        .on_press(Message::CurrentScreen(0))
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(if self1.currentscreen == 0 {
                    Length::FillPortion(3)
                } else {
                    Length::FillPortion(2)
                })
                .height(Length::Fill),
            )
            .push(
                Container::new(
                    Button::new(content1(String::from("Search"), '\u{f002}'))
                        // .style(if self1.currentpage == 1 {
                        //     nav_style::buttonStyle::FilterSelected
                        // } else {
                        //     nav_style::buttonStyle::FilterActive
                        // })
                        .on_press(Message::CurrentScreen(1))
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(if self1.currentscreen == 1 {
                    Length::FillPortion(3)
                } else {
                    Length::FillPortion(2)
                })
                .height(Length::Fill),
            )
            .push(
                Container::new(
                    Button::new(content1(String::from("Post"), '\u{e2df}'))
                        // .style(if self1.currentpage == 2 {
                        //     nav_style::buttonStyle::FilterSelected
                        // } else {
                        //     nav_style::buttonStyle::Special
                        // })
                        .on_press(Message::CurrentScreen(2))
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(if self1.currentscreen == 2 {
                    Length::FillPortion(3)
                } else {
                    Length::FillPortion(2)
                })
                .height(Length::Fill),
            )
            .push(
                Container::new(
                    Button::new(content1(String::from("Chat"), '\u{f4b6}'))
                        // .style(if self1.currentpage == 3 {
                        //     nav_style::buttonStyle::FilterSelected
                        // } else {
                        //     nav_style::buttonStyle::FilterActive
                        // })
                        .on_press(Message::CurrentScreen(3))
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(if self1.currentscreen == 3 {
                    Length::FillPortion(3)
                } else {
                    Length::FillPortion(2)
                })
                .height(Length::Fill),
            ), // .push(
               //     Container::new(
               //         Button::new(content1(String::from("User"), '\u{f007}'))
               //             // .style(if self1.currentpage == 4 {
               //             //     nav_style::buttonStyle::FilterSelected
               //             // } else {
               //             //     nav_style::buttonStyle::FilterActive
               //             // })
               //             .on_press(Message::FifthButtonPressed)
               //             .width(Length::Fill)
               //             .height(Length::Fill),
               //     )
               //     .width(if self1.currentscreen == 4 {
               //         Length::FillPortion(3)
               //     } else {
               //         Length::FillPortion(2)
               //     })
               //     .height(Length::Fill),
               // ),
    )
    .height(Length::Units(60))
    .width(Length::Fill)
    // .style(nav_style::containerStyle::navContainer)
    .into();
    //return content1("".to_string(), '\u{f015}');
}
