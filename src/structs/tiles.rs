#[derive(Debug)]
pub struct Tiles {
    pub images: Vec<String>,
}

impl Tiles {
    pub fn new() -> Self {
        let asset_path: String = String::from("resources/road_tiles/");

        Tiles {
            images: vec![
                format!("{asset_path}blank.png"),
                format!("{asset_path}down.png"),
                format!("{asset_path}left.png"),
                format!("{asset_path}right.png"),
                format!("{asset_path}up.png"),
            ],
        }
    }
}
