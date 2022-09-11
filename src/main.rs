use iced::{executor, Application, Command, Container, Element, Image, Length, Settings};
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
        let image = Image::new("resources/autotile_tileset.png")
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(image)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
