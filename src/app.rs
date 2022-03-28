use std::io::Error;

use crate::collisions::collision_handelers::collision_handeler;
use crate::shape::shapes::{self, Shape};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect;
use std::time::Duration;

pub fn run() -> Result<(), Error> {
    /*
        a bunch of vectors that store each shape so we can keep track of thier position and do collisions
    */
    let mut square_objects: Vec<shapes::Square> = vec![];
    let mut circle_objects: Vec<shapes::Circle> = vec![];
    let mut triangle_objects: Vec<shapes::Triangle> = vec![];
    let mut trapaziod_objects: Vec<shapes::Trapaziod> = vec![];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Floating Shapes", 500, 500)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let first_shape: shapes::Square = shapes::Square::new_shape((500, 500), (-3, 3));

    square_objects.push(first_shape);

    let rect = rect::Rect::new(
        square_objects[0].get_location().0,
        square_objects[0].get_location().1,
        100,
        100,
    );

    canvas.fill_rect(rect).expect("error drawing");

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //TODO:insert function to handel creation of shapes and updating their locations
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let curr_location = square_objects[0].get_location();
        let curr_direction = square_objects[0].get_direction();

        square_objects[0].change_direction(collision_handeler(
            curr_direction,
            curr_location,
            (500, 500),
        ));

        square_objects[0].update_position();

        let rect = rect::Rect::new(
            square_objects[0].get_location().0,
            square_objects[0].get_location().1,
            100,
            100,
        );
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        canvas.fill_rect(rect).expect("error drawing");

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
