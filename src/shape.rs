pub mod shapes {

    use crate::random_data::{rng, rng_but_color, rng_direction};
    use sdl2::pixels::Color;
    /*
        Shape Structs:::::::
    */
    pub struct Square {
        color: Color,
        height: f32,
        width: f32,
        xy: (i32, i32),
        dxdy: (i32, i32),
    }

    pub struct Triangle {
        color: Color,
        height: f32,
        width: f32,
        xy: (i32, i32),
        dxdy: (i32, i32),
    }

    pub struct Circle {
        color: Color,
        radius: f32,
        xy: (i32, i32),
        dxdy: (i32, i32),
    }

    pub struct Trapaziod {
        color: Color,
        a: f32,
        b: f32,
        height: f32,
        xy: (i32, i32),
        dxdy: (i32, i32),
    }

    /*
        Shape Struct functions
    */

    //every shape will have a function to create itself and move itself
    //requires the width and height of the window the shape is drawn on so it doesnt get drawn off the screen
    //Also requires a tupal of the desired change in x and y. This allows for the shapes to have different speeds as they are created
    //functions to change the shapes location, direction and speed are also included
    pub trait Shape {
        fn new_shape(bounds: (i32, i32), delta: (i32, i32)) -> Self;
        fn update_position(&mut self);
        fn change_direction(&mut self, new_deltas: (i32, i32));
        fn get_location(&mut self) -> (i32, i32);
        fn get_direction(&mut self) -> (i32, i32);
    }

    impl Shape for Square {
        fn new_shape(limits: (i32, i32), delta: (i32, i32)) -> Self {
            Self {
                color: rng_but_color(),
                height: 100.0,
                width: 100.0,
                xy: rng(limits),
                dxdy: rng_direction(delta),
            }
        }

        fn update_position(&mut self) {
            self.xy = (self.xy.0 + self.dxdy.0, self.xy.1 + self.dxdy.1);
        }

        fn change_direction(&mut self, new_deltas: (i32, i32)) {
            self.dxdy = new_deltas;
        }

        fn get_location(&mut self) -> (i32, i32) {
            self.xy
        }

        fn get_direction(&mut self) -> (i32, i32) {
            self.dxdy
        }
    }

    impl Shape for Triangle {
        fn new_shape(bounds: (i32, i32), delta: (i32, i32)) -> Self {
            todo!()
        }

        fn update_position(&mut self) {
            todo!()
        }

        fn change_direction(&mut self, new_deltas: (i32, i32)) {
            todo!()
        }

        fn get_location(&mut self) -> (i32, i32) {
            todo!()
        }

        fn get_direction(&mut self) -> (i32, i32) {
            todo!()
        }
    }

    impl Shape for Circle {
        fn new_shape(bounds: (i32, i32), delta: (i32, i32)) -> Self {
            todo!()
        }

        fn update_position(&mut self) {
            todo!()
        }

        fn change_direction(&mut self, new_deltas: (i32, i32)) {
            todo!()
        }

        fn get_location(&mut self) -> (i32, i32) {
            todo!()
        }

        fn get_direction(&mut self) -> (i32, i32) {
            todo!()
        }
    }

    impl Shape for Trapaziod {
        fn new_shape(bounds: (i32, i32), delta: (i32, i32)) -> Self {
            todo!()
        }

        fn update_position(&mut self) {
            todo!()
        }

        fn change_direction(&mut self, new_deltas: (i32, i32)) {
            todo!()
        }

        fn get_location(&mut self) -> (i32, i32) {
            todo!()
        }

        fn get_direction(&mut self) -> (i32, i32) {
            todo!()
        }
    }
}

pub mod collisions {}

#[cfg(test)]
mod tests {

    use super::shapes::Square;
    use crate::shape::shapes::Shape;

    #[test]
    fn test_square() {
        let mut test_thing: Square = super::shapes::Square::new_shape((1000, 1000), (-3, 3));

        //initial coordinates, these will be used later to test if the update position code works
        let initial_x = test_thing.get_location().0;
        let initial_y = test_thing.get_location().1;

        //random dxdy, these will also be used later to make sure the update position code works
        let dx = test_thing.get_direction().0;
        let dy = test_thing.get_direction().1;

        //make sure location is within what the canvas would be
        assert!(test_thing.get_location().0 < 1000 && test_thing.get_location().1 < 1000);

        assert!(test_thing.get_direction().0 == dx && test_thing.get_direction().1 == dy);

        //now call the update position function to simulate the object moving as the canvas updates
        test_thing.update_position();

        // /*
        //     at this point the object moved. In this case it moved to the right 3 pixels and down three pixels, so the position should be
        //     +3 of what the origonal coordinates were
        // */
        assert_eq!(test_thing.get_location().0, initial_x + dx);
        assert_eq!(test_thing.get_location().1, initial_y + dy);
    }
}
