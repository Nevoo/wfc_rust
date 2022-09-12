use iced::{Color, Image, Point, Size};

use iced::canvas::{self, Cache, Fill, Path};

use std::collections::HashSet;

pub struct Grid {
    cache: Cache,
    pub cells: HashSet<Cell>,
}

impl<Message> canvas::Program<Message> for Grid {
    fn draw(&self, bounds: iced::Rectangle, cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        let grid = self.cache.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());

            frame.fill(&background, Color::BLACK);

            frame.with_save(|frame| {
                frame.scale(Cell::SIZE as f32);

                for cell in &self.cells {
                    frame.fill_rectangle(
                        Point::new(cell.x as f32, cell.y as f32),
                        Size::UNIT,
                        Color::WHITE,
                    )
                }
            });
        });

        vec![grid]
    }
}

impl std::iter::FromIterator<Cell> for Grid {
    fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
        Self {
            cells: iter.into_iter().collect(),
            ..Self::default()
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::from_dimension(10, 10)
    }
}

impl Grid {
    pub fn from_dimension(width: isize, height: isize) -> Self {
        let mut grid: Vec<Cell> = Vec::new();

        for x in 0..width {
            for y in 0..height {
                grid.push(Cell { x, y })
            }
        }

        Self {
            cells: grid.into_iter().collect(),
            cache: Cache::default(),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Cell {
    x: isize,
    y: isize,
}

impl Cell {
    const SIZE: usize = 50;
}
