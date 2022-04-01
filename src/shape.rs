pub mod shapes {

    use crate::random_data::{rng, rng_but_color, rng_direction};
    use sdl2::pixels::Color;
    /*
        Shape Structs:::::::
    */
    pub struct Square {
        color: Color,
        dimensions: (i32, i32),
        xy: (i32, i32),
        dxdy: (i32, i32),
    }

    pub struct Circle {
        color: Color,
        radius: (i32, i32),
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
        fn new_shape(bounds: (i32, i32), delta: (i32, i32), dimensions: (i32, i32)) -> Self;
        fn update_position(&mut self);
        fn change_direction(&mut self, new_deltas: (i32, i32));
        fn get_dimensions(&mut self) -> (i32, i32);
        fn get_location(&mut self) -> (i32, i32);
        fn get_direction(&mut self) -> (i32, i32);
        fn get_color(&mut self) -> Color;
    }

    impl Shape for Square {
        fn new_shape(limits: (i32, i32), delta: (i32, i32), dimensions: (i32, i32)) -> Self {
            Self {
                color: rng_but_color(),
                dimensions: dimensions,
                xy: rng((limits.0 - 100, limits.1 - 100)),
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

        fn get_dimensions(&mut self) -> (i32, i32) {
            self.dimensions
        }

        fn get_color(&mut self) -> Color {
            self.color
        }
    }

    impl Shape for Circle {
        fn new_shape(limits: (i32, i32), delta: (i32, i32), dimensions: (i32, i32)) -> Self {
            Self {
                color: rng_but_color(),
                radius: dimensions,
                xy: rng((limits.0 - 100, limits.1 - 100)),
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

        fn get_dimensions(&mut self) -> (i32, i32) {
            self.radius
        }
        fn get_color(&mut self) -> Color {
            self.color
        }
    }
}

#[cfg(test)]
mod tests {

    use super::shapes::Square;
    use crate::shape::shapes::Shape;

    #[test]
    fn test_square() {
        let mut test_thing: Square =
            super::shapes::Square::new_shape((1000, 1000), (-3, 3), (100, 100));

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

        assert_eq!(test_thing.get_dimensions().0, 100);
        assert_eq!(test_thing.get_dimensions().1, 100);
    }
}
