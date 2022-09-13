use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let window = video_subsystem
        .window("rust-sdl2 wave function collapse", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
        .load_texture("resources/road_tiles/down.png")
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let rect = Rect::new(0, 0, 300, 300);

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        canvas.clear();

        let (width, height) = canvas.output_size().unwrap();

        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = Point::new(0, 0) + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, rect.width(), rect.height());
        canvas.copy(&texture, rect, screen_rect).unwrap();

        canvas.present();
    }
}
