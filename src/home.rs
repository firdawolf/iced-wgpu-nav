use iced_native::{
    command::Command,
    program::Program,
    widget::{button, column::Column, row::Row, slider, text::Text, Container},
    Alignment, Color, Element, Length,
};
use iced_wgpu::Renderer;
#[path = "icons/icons.rs"]
mod icons;
use icons::icon;
#[path = "navbar/mod.rs"]
mod navbar;
use navbar::navbar;
#[path = "public_state/mod.rs"]
mod public_state;
pub use public_state::nav_state::{Home, HomeScreenState, Message, PublicScreenState};
#[path = "homescreen/homescreen.rs"]
mod homescreen;
use homescreen::HomeScreen;

// use iced_winit::widget::TextInput;
// use iced_winit::widget::{ Column, Row, Text};

impl Home {
    pub fn new() -> Home {
        Home::Loaded(PublicScreenState {
            currentscreen: 0,
            home_screen_state: HomeScreenState {
                fps: Default::default(),
            },
        })
    }
}

impl Program for Home {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Home::Loading => {}
            Home::Loaded(loadedData) => match message {
                Message::CurrentScreen(screen) => {
                    loadedData.currentscreen = screen;
                }
                Message::UpdateHomeScreen(homeScreenState) => {
                    loadedData.home_screen_state.fps = homeScreenState.fps;
                }
            },
        }

        Command::none()
    }

    fn view(self: &Home) -> Element<Message, Renderer> {
        match self {
            Home::Loading => Container::new(Text::new("Loading ..."))
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
            Home::Loaded(public_state) => Container::new(
                Column::new()
                    .align_items(Alignment::Center)
                    // .push(match buttomNavbar.currentpage {
                    //     0 => screen.homescreen(),
                    //     1 => screen.secondpage(),
                    //     x => screen.homescreen(),
                    // })
                    .push(match public_state.currentscreen {
                        0 => HomeScreen(&public_state.home_screen_state),
                        _ => Container::new(Text::new(format!(
                            "Loaded Screen {}",
                            public_state.currentscreen
                        )))
                        .height(Length::Fill)
                        .width(Length::Fill),
                    })
                    .push(navbar(public_state)),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into(),
        }

        //let surface_windows = Compositor::new(settings, compatible_window)
        // Container::new(
        //     Row::new()
        //         .width(Length::Fill)
        //         .height(Length::Fill)
        //         .align_items(Alignment::Start)
        //         .push(
        //             Column::new()
        //                 .width(Length::Fill)
        //                 .align_items(Alignment::Start)
        //                 .push(
        //                     Column::new().padding(10).spacing(10).push(
        //                         Text::new(format!("Render at : {} fps", self.text.to_string()))
        //                             .size(28),
        //                     ),
        //                 )
        //                 // .push(slider::Slider::new(
        //                 //        &mut self.sliderstate,
        //                 //        0.0..=100.0,
        //                 //        self.background_color.g,
        //                 //        move |b| {},
        //                 //    )),
        //                 .push(
        //                     button::Button::new(
        //                         Column::new()
        //                             .width(Length::Fill)
        //                             .align_items(Alignment::Start)
        //                             .push(Text::new("Press Here"))
        //                             .push(icon('\u{f004}', 20)),
        //                     )
        //                     .on_press(Message::ButtonPressed),
        //                 ),
        //         ),
        // )
        // .height(Length::Fill)
        // .width(Length::Fill)
        // .into()
    }
}
