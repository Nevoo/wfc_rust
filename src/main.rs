use iced::{
    executor, Application, Column, Command, Container, Element, Image, Length, Row, Settings,
};

mod structs;
use structs::Tiles;

fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example;

impl Application for Example {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Example, Command<Self::Message>) {
        (Example, Command::none())
    }

    fn title(&self) -> String {
        String::from("Example application")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let base_tiles = Tiles::new();

        println!("{:?}", base_tiles);

        let blank_tile = Image::new(base_tiles.blank)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        let down_tile = Image::new(base_tiles.down)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        let left_tile = Image::new(base_tiles.left)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        let right_tile = Image::new(base_tiles.right)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        let up_tile = Image::new(base_tiles.up)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        Container::new(Column::with_children(vec![
            blank_tile, down_tile, left_tile, right_tile, up_tile,
        ]))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
