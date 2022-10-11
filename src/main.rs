use iced::{text_input, Alignment, Column, Container, Length, Sandbox, Settings, Text, TextInput};

// https://github.com/iced-rs/iced/blob/0.4/examples/styling/src/main.rs

fn main() -> iced::Result {
    Program::run(Settings::default())
}

#[derive(Default)]
struct Program {
    input: text_input::State,
    input_value: String,
    display_value: String,
}

#[derive(Clone, Debug)]
enum Message {
    InputChanged(String),
    InputInputted,
}

impl Sandbox for Program {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Program".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::InputInputted => {
                self.input_value = {
                    self.display_value += &self.input_value;
                    "".into()
                }
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let content = Column::new()
            .push(
                TextInput::new(
                    &mut self.input,
                    "Type something...",
                    &self.input_value,
                    Message::InputChanged,
                )
                .size(64)
                .padding(8)
                .style(style::TextInput)
                .on_submit(Message::InputInputted),
            )
            .push(Text::new(&self.display_value))
            .padding(16)
            .spacing(16)
            .align_items(Alignment::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            // .center_y()
            .style(style::Container)
            .into()
    }
}

mod style {
    use iced::{container, text_input, Color};

    const SURFACE: Color = Color::from_rgb(
        0x40 as f32 / 255.0,
        0x44 as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    const ACCENT: Color = Color::from_rgb(
        0xA1 as f32 / 255.0,
        0x64 as f32 / 255.0,
        0xFC as f32 / 255.0,
    );

    const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    const HOVERED: Color = Color::from_rgb(
        0x67 as f32 / 255.0,
        0x7B as f32 / 255.0,
        0xC4 as f32 / 255.0,
    );

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            }
        }
    }

    pub struct TextInput;

    impl text_input::StyleSheet for TextInput {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: SURFACE.into(),
                border_radius: 8.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: ACCENT,
                ..self.active()
            }
        }

        fn hovered(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: Color { a: 0.3, ..ACCENT },
                ..self.focused()
            }
        }

        fn placeholder_color(&self) -> Color {
            Color::from_rgb(0.4, 0.4, 0.4)
        }

        fn value_color(&self) -> Color {
            Color::WHITE
        }

        fn selection_color(&self) -> Color {
            ACTIVE
        }
    }
}
