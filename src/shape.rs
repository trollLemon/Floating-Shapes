pub mod shapes {

    use crate::random_data::{rng, rng_but_color};
    use sdl2::pixels::Color;
    /*
        Shape Structs:::::::
    */
    pub struct Square {
        color: Color,
        height: f32,
        width: f32,
        x: i32,
        y: i32,
    }

    pub struct Triangle {
        color: Color,
        height: f32,
        width: f32,
        x: i32,
        y: i32,
    }

    pub struct Circle {
        color: Color,
        radius: f32,
        x: i32,
        y: i32,
    }

    pub struct Trapaziod {
        color: Color,
        a: f32,
        b: f32,
        height: f32,
        x: i32,
        y: i32,
    }

    /*
        Shape Struct functions
    */

    //every shape will have a function to create itself and move itself
    trait Shape {
        fn new(&self) -> Self;
        fn update_position(&mut self, x: i32, y: i32);
    }

    impl Shape for Square {
        fn new(&self) -> Self {
            Self {
                color: todo!(),
                height: todo!(),
                width: todo!(),
                x: todo!(),
                y: todo!(),
            }
        }

        fn update_position(&mut self, x: i32, y: i32) {
            todo!()
        }
    }

    impl Shape for Triangle {
        fn new(&self) -> Self {
            todo!()
        }

        fn update_position(&mut self, x: i32, y: i32) {
            todo!()
        }
    }

    impl Shape for Circle {
        fn new(&self) -> Self {
            todo!()
        }

        fn update_position(&mut self, x: i32, y: i32) {
            todo!()
        }
    }

    impl Shape for Trapaziod {
        fn new(&self) -> Self {
            todo!()
        }

        fn update_position(&mut self, x: i32, y: i32) {
            todo!()
        }
    }
}

pub mod collisions {}

#[cfg(test)]
mod tests {}
