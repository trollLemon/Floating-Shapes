use std::io::Error;

use crate::circle;

use crate::circle::circle::Circle;
use crate::shape::shapes::Shape;
use crate::square::square::Square;

use sdl2::event::Event;

use sdl2::keyboard::Keycode;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Canvas;

use sdl2::video::Window;

use std::time::Duration;

use rand::Rng;

const DIMENSIONS: (u32, u32) = (1000, 1000);
const BOUNDS: (i32, i32) = (1000, 1000);

pub fn run() -> Result<(), Error> {
    /*
        a bunch of vectors that store each shape so we can keep track of thier position and do collisions
    */
    let mut square_objects: Vec<Square> = vec![];
    let mut circle_objects: Vec<Circle> = vec![];
    let mut delete_list: Vec<i32> = vec![]; //used to figure out wether to delete a square or a circle (0 = square, 1 = circle)

    let mut width_and_height: (i32, i32) = (50, 50);
    let mut radius = 25;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Floating Shapes", DIMENSIONS.0, DIMENSIONS.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas
        .filled_circle(100, 100, 100, Color::RED)
        .expect("error drawing");
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => {
                    let zero_or_one = rand::thread_rng().gen_range(0..2);

                    if zero_or_one == 0 {
                        square_creator(width_and_height, &mut square_objects);
                        delete_list.push(0);
                    } else {
                        circle_creator(radius, &mut circle_objects);
                        delete_list.push(1);
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    let id = delete_list.pop();

                    match id {
                        Some(0) => {
                            delete_a_square(&mut square_objects);
                        }
                        Some(1) => {
                            delete_a_circle(&mut circle_objects);
                        }
                        _ => {}
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    clear_canvas(&mut square_objects, &mut circle_objects);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    increase_width(&mut width_and_height);
                    increase_radius(&mut radius);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    increase_height(&mut width_and_height);
                    increase_radius(&mut radius);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    decrease_width(&mut width_and_height);
                    decrease_radius(&mut radius);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    decrease_height(&mut width_and_height);
                    decrease_radius(&mut radius);
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
        shape_handeler(&mut square_objects, &mut circle_objects, &mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
    Ok(())
}

fn shape_handeler(
    squares: &mut Vec<Square>,
    circles: &mut Vec<Circle>,
    canvas: &mut Canvas<Window>,
) {
    shape_updater(squares, circles);
    shape_drawer(squares, circles, canvas);
}

fn shape_drawer(squares: &mut Vec<Square>, circles: &mut Vec<Circle>, canvas: &mut Canvas<Window>) {
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
    for shape in circles {
        canvas.set_draw_color(shape.get_color());
        canvas
            .filled_circle(
                shape.get_location().0.try_into().unwrap(),
                shape.get_location().1.try_into().unwrap(),
                shape.get_radius().try_into().unwrap(),
                shape.get_color(),
            )
            .expect("error drawing");
    }
}

fn delete_a_square(squares: &mut Vec<Square>) {
    squares.pop();
}

fn delete_a_circle(circles: &mut Vec<Circle>) {
    circles.pop();
}

fn clear_canvas(squares: &mut Vec<Square>, circles: &mut Vec<Circle>) {
    squares.clear();
    circles.clear();
}

fn shape_updater(squares: &mut Vec<Square>, circles: &mut Vec<Circle>) {
    for shape in squares {
        shape.change_direction();
        shape.update_position();
    }

    for shape in circles {
        shape.change_direction();
        shape.update_position();
    }
}

fn square_creator(width_and_height: (i32, i32), squares: &mut Vec<Square>) {
    let shape: Square = Square::new_shape(
        (BOUNDS.0 - width_and_height.0, BOUNDS.1 - width_and_height.1),
        (-5, 5),
        width_and_height,
    );
    squares.push(shape);
}

fn circle_creator(radius: i32, circles: &mut Vec<circle::circle::Circle>) {
    let shape: Circle = Circle::new_shape(
        (BOUNDS.0 - radius, BOUNDS.1 - radius),
        (-5, 5),
        (radius, radius),
    );
    circles.push(shape);
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

fn increase_radius(radius: &mut i32) {
    *radius = *radius + 1;
}

fn decrease_radius(radius: &mut i32) {
    if *radius > 0 {
        *radius = *radius - 1;
    }
}
