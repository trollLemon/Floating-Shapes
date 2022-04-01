use std::io::Error;

use crate::collisions::collision_handelers::collision_handeler;

use crate::shape::shapes::{self, Shape};

use sdl2::event::Event;

use sdl2::keyboard::Keycode;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Canvas;

use sdl2::video::Window;

use std::time::Duration;

pub fn run() -> Result<(), Error> {
    /*
        a bunch of vectors that store each shape so we can keep track of thier position and do collisions
    */
    let mut square_objects: Vec<shapes::Square> = vec![];
    let mut _circle_objects: Vec<shapes::Circle> = vec![];
    let mut width_and_height: (i32, i32) = (100, 100);
    const _RADIUS: i32 = 50;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Floating Shapes", 1000, 1000)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas
        .filled_circle(100, 100, 100, Color::RED)
        .expect(r#"oops"#);
    //canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => {
                    shape_creator(width_and_height, &mut square_objects);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    delete_a_shape(&mut square_objects);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    clear_canvas(&mut square_objects);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    increase_width(&mut width_and_height);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    increase_height(&mut width_and_height);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    decrease_width(&mut width_and_height);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    decrease_height(&mut width_and_height);
                }

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
        shape_handeler(&mut square_objects, &mut canvas, width_and_height);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

fn shape_handeler(
    squares: &mut Vec<shapes::Square>,
    canvas: &mut Canvas<Window>,
    width_and_height: (i32, i32),
) {
    shape_updater(squares, width_and_height);
    shape_drawer(squares, canvas);
}

fn shape_drawer(squares: &mut Vec<shapes::Square>, canvas: &mut Canvas<Window>) {
    for shape in squares {
        let rect = rect::Rect::new(
            shape.get_location().0,
            shape.get_location().1,
            shape.get_dimensions().0.try_into().unwrap(),
            shape.get_dimensions().1.try_into().unwrap(),
        );

        canvas.set_draw_color(shape.get_color());
        canvas.fill_rect(rect).expect("error drawing");
    }
}

fn delete_a_shape(squares: &mut Vec<shapes::Square>) {
    squares.pop();
}

fn clear_canvas(squares: &mut Vec<shapes::Square>) {
    squares.clear();
}

fn shape_updater(squares: &mut Vec<shapes::Square>, width_and_height: (i32, i32)) {
    for shape in squares {
        let curr_location = shape.get_location();
        let curr_direction = shape.get_direction();

        shape.change_direction(collision_handeler(
            curr_direction,
            curr_location,
            (1000, 1000),
            width_and_height,
        ));

        shape.update_position();
    }
}

fn shape_creator(width_and_height: (i32, i32), squares: &mut Vec<shapes::Square>) {
    let shape: shapes::Square = shapes::Square::new_shape(
        (1000 - width_and_height.0, 1000 - width_and_height.1),
        (-5, 5),
        width_and_height,
    );
    squares.push(shape);
}

fn increase_width(dimensions: &mut (i32, i32)) {
    dimensions.0 = dimensions.0 + 1;
}
fn increase_height(dimensions: &mut (i32, i32)) {
    dimensions.1 = dimensions.1 + 1;
}
fn decrease_width(dimensions: &mut (i32, i32)) {
    if dimensions.0 > 0 {
        dimensions.0 = dimensions.0 - 1;
    }
}
fn decrease_height(dimensions: &mut (i32, i32)) {
    if dimensions.1 > 0 {
        dimensions.1 = dimensions.1 - 1;
    }
}
