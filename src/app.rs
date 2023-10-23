use iced::widget::{self, *};
use iced::{Alignment, Application, Color, Command, Length};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateInput(String),
}

struct InputHandler {
    value: String,
    placeholder: &'static str,
}

pub struct App {
    input: InputHandler,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                input: InputHandler {
                    value: String::new(),
                    placeholder: "Type something...",
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "PyE Chat".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::UpdateInput(input) => self.input.value = input,
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let elements: iced::Element<Self::Message, iced::Renderer<Self::Theme>> = widget::column![
            container(
                row![
                    text("#general"),
                    horizontal_space(Length::Fill),
                    button("connect").padding(10)
                ]
                .align_items(Alignment::Center)
            )
            .height(Length::Shrink), // info + connect
            container(
                widget::column![horizontal_space(Length::Fill), vertical_space(Length::Fill)]
                    .height(Length::Fill)
                    .height(Length::Fill)
            )
            .height(Length::Fill), // messages
            container(row![
                text_input(self.input.placeholder, &self.input.value)
                    .padding(10)
                    .on_input(Message::UpdateInput),
                button("Send").padding(10)
            ])
            .height(Length::Shrink) // input + send button
        ]
        .padding(20)
        .height(Length::Fill)
        .width(Length::Fill)
        .into();

        // elements
        elements.explain(Color::from_rgb(255.0, 0.0, 0.0))
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }
}
