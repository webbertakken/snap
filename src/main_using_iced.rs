use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::widget::{button, container, row, text, Column, Text};
use iced::{Alignment, Application, Command, Element, Length, Settings};

pub fn main() -> iced::Result {
    SnapUI::run(Settings::default())
}

struct SnapUI {}

#[derive(Debug, Clone)]
enum Message {
    TakeScreenshot,
}

impl Application for SnapUI {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (SnapUI, Command<Message>) {
        (SnapUI {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Snap")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TakeScreenshot => {
                println!("Taking screenshot");
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let button = |label| {
            button(text(label).horizontal_alignment(alignment::Horizontal::Center))
                .padding(10)
                .width(Length::Units(100))
        };

        let take_screenshot_button = button("Take Screenshot")
            .style(theme::Button::Primary)
            .width(Length::Units(200))
            .on_press(Message::TakeScreenshot);

        let controls = row![take_screenshot_button].spacing(20);

        let content  = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(Text::new("Screenshot App").size(50))
            .push(controls)
            // .push(match &self.screenshot_result {
            //     Some(result) => Text::new(result.to_string()),
            //     None => Text::new(""),
            // })
            ;

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        match dark_light::detect() {
            dark_light::Mode::Dark => Theme::Dark,
            dark_light::Mode::Light => Theme::Light,
            dark_light::Mode::Default => Theme::Light,
        }
    }
}
