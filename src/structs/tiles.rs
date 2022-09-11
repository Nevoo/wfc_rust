#[derive(Debug)]
pub struct Tiles {
    pub blank: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub up: String,
}

impl Tiles {
    pub fn new() -> Self {
        let base_path = String::from("resources/road_tiles/");

        Tiles {
            blank: format!("{base_path}blank.png"),
            down: format!("{base_path}down.png"),
            left: format!("{base_path}left.png"),
            right: format!("{base_path}right.png"),
            up: format!("{base_path}up.png"),
        }
    }
}
