use iced::{
    alignment::{self, Horizontal},
    executor,
    widget::{
        self, button, button::State, text_input, Button, Column, Container, Row, Text, TextInput,
    },
    Application, Command, Element, Length, Settings,
};

pub fn main() -> iced::Result {
    StegText::run(Settings::default())
}

struct StegText {
    secret_input_state: text_input::State,
    secret_input: String,
    cover_input_state: text_input::State,
    cover_input: String,
    output_state: text_input::State,
    output: String,
    encode_button_state: button::State,
    is_encoding: bool,
}

#[derive(Debug, Clone)]
enum Message {
    SecretInputChanged(String),
    CoverInputChanged(String),
    ToggleEncode,
}

impl Application for StegText {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                secret_input_state: text_input::State::new(),
                secret_input: String::new(),
                cover_input_state: text_input::State::new(),
                cover_input: String::new(),
                output_state: text_input::State::new(),
                output: String::from("The output will appear here"),
                encode_button_state: button::State::new(),
                is_encoding: true,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("StegText")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SecretInputChanged(value) => {
                self.secret_input = value;
            }
            Message::CoverInputChanged(value) => {
                self.cover_input = value;
            }
            Message::ToggleEncode => {
                let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
                if self.is_encoding {
                    // Encoding logic
                    match stego_wps::encode(&self.cover_input) {
                        Ok(encoded) => {
                            self.output = encoded
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(" ")
                        }
                        Err(e) => self.output = format!("Error encoding: {}", e),
                    }
                } else {
                    // Decoding logic
                    let numbers = self
                        .cover_input
                        .split_whitespace()
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect::<Vec<_>>();
                    match stego_wps::decode(&numbers, character_set) {
                        Ok(decoded) => self.output = decoded,
                        Err(e) => self.output = format!("Error decoding: {}", e),
                    }
                }
                self.is_encoding = !self.is_encoding;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let secret_input = TextInput::new(
            &mut self.secret_input_state,
            "Enter secret text...",
            &self.secret_input,
            Message::SecretInputChanged,
        )
        .padding(10)
        .size(20);

        let cover_input = TextInput::new(
            &mut self.cover_input_state,
            "Enter cover text...",
            &self.cover_input,
            Message::CoverInputChanged,
        )
        .padding(10)
        .size(20);

        let output = TextInput::new(&mut self.output_state, "", &self.output)
            .padding(10)
            .size(20)
            .read_only(true);

        let encode_button = Button::new(
            &mut self.encode_button_state,
            Text::new(if self.is_encoding { "Encode" } else { "Decode" }).size(20),
        )
        .on_press(Message::ToggleEncode);

        let content = Column::new()
            .push(secret_input)
            .push(cover_input)
            .push(output)
            .push(encode_button);

        Container::new(content).into()
    }
}
