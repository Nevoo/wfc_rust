use iced::{executor, Application, Column, Command, Container, Element, Image, Length, Settings};

use iced::canvas::Canvas;

mod structs;
use structs::{Grid, Tiles};

fn main() -> iced::Result {
    WaveFunctionCollapse::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Debug)]
enum Message {}

#[derive(Default)]
struct WaveFunctionCollapse {
    grid: Grid,
}

impl Application for WaveFunctionCollapse {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Example application")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let canvas: Element<Message> = Canvas::new(&mut self.grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        let base_tiles = Tiles::new();

        // let images = base_tiles
        //     .images
        //     .iter()
        //     .map(|tile| {
        //         Image::new(tile)
        //             .width(Length::Fill)
        //             .height(Length::Fill)
        //             .into()
        //     })
        //     .collect();

        Container::new(Column::new().push(canvas))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
