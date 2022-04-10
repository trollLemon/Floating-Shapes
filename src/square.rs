pub mod square {

    use crate::random_data::{rng, rng_but_color, rng_direction};
    use crate::shape::shapes::Shape;
    use sdl2::pixels::Color;

    pub struct Square {
        color: Color,
        dimensions: (i32, i32),
        xy: (i32, i32),
        dxdy: (i32, i32),
        bounds: (i32, i32),
    }

    impl Shape for Square {
        fn new_shape(limits: (i32, i32), delta: (i32, i32), dimensions: (i32, i32)) -> Self {
            Self {
                color: rng_but_color(),
                dimensions: dimensions,
                xy: rng((limits.0 - 100, limits.1 - 100)),
                dxdy: rng_direction(delta),
                bounds: limits,
            }
        }

        fn update_position(&mut self) {
            self.xy = (self.xy.0 + self.dxdy.0, self.xy.1 + self.dxdy.1);
        }

        fn change_direction(&mut self) {
            if self.is_collidable() {
                self.dxdy =
                    self.collision_handeler(self.dxdy, self.xy, self.bounds, self.dimensions);
            }
        }

        fn get_location(&mut self) -> (i32, i32) {
            self.xy
        }

        fn get_direction(&mut self) -> (i32, i32) {
            self.dxdy
        }

        fn get_color(&mut self) -> Color {
            self.color
        }
    }

    impl Square {
        pub fn collision_handeler(
            &mut self,
            current_dxdy: (i32, i32),
            position: (i32, i32),
            bounds: (i32, i32),
            dimensions: (i32, i32),
        ) -> (i32, i32) {
            //top left corner collision
            if position.0 <= 0 && position.1 <= 0 {
                (current_dxdy.0 * -1, current_dxdy.1 * -1)
            }
            //top right corner collision
            else if position.0 >= bounds.0 - dimensions.0 && position.1 <= 0 {
                (current_dxdy.0 * -1, current_dxdy.1 * -1)
            }
            //top of the screen collision
            else if position.0 < bounds.0 && position.1 <= 0 {
                (current_dxdy.0, current_dxdy.1 * -1)
            }
            //bottom left corner collision
            else if position.0 <= 0 && position.1 >= bounds.1 {
                (current_dxdy.0 * -1, current_dxdy.1 * -1)
            }
            //bottom right corner collision
            else if position.0 >= bounds.0 - dimensions.0 && position.1 >= bounds.1 - dimensions.1
            {
                (current_dxdy.0 * -1, current_dxdy.1 * -1)
            }
            //bottom collision
            else if position.0 < bounds.0 && position.1 >= bounds.1 {
                (current_dxdy.0, current_dxdy.1 * -1)
            }
            //right side collision
            else if position.0 >= bounds.0 && position.1 < bounds.1 {
                (current_dxdy.0 * -1, current_dxdy.1)
            }
            //left side collision
            else if position.0 <= 0 && position.1 < bounds.1 {
                (current_dxdy.0 * -1, current_dxdy.1)
            }
            //No collision: the function should return the same dxdy since its not hitting anything
            else {
                current_dxdy
            }
        }

        /*If the square is nowhere near the edges of the screen, we dont need to check for collisions with the screen*/
        pub fn collision(&mut self) -> bool {
            //if the coordinates of the shape are near the dimensions, then it may collide soon, so return true
            if (self.get_location().0 > self.dimensions.0
                && self.get_location().1 > self.dimensions.1)
                && (self.get_location().0 < self.bounds.0 - self.get_dimensions().0
                    && self.get_location().1 < self.bounds.1 - self.get_dimensions().1)
            {
                false
            } else {
                true
            }
        }

        pub fn is_collidable(&mut self) -> bool {
            self.collision()
        }

        pub fn get_dimensions(&mut self) -> (i32, i32) {
            self.dimensions
        }

        //used for testing, not for movement within the app
        pub fn _change_location(&mut self, _x_and_y: (i32, i32)) {
            self.xy = _x_and_y;
        }
    }
}

#[cfg(test)]
mod tests {

    //collision tests

    use crate::shape::shapes::Shape;

    use super::square::Square;

    const BOUNDS: (i32, i32) = (1000, 1000);

    const W_AND_HEIGHT: (i32, i32) = (100, 100);

    const POS_1: (i32, i32) = (900, 0);
    const POS_2: (i32, i32) = (0, 1000);
    const POS_3: (i32, i32) = (900, 900);
    const POS_4: (i32, i32) = (0, 0);
    const POS_5: (i32, i32) = (100, 500);
    const POS_6: (i32, i32) = (900, 500);
    const POS_7: (i32, i32) = (500, 0);
    const POS_8: (i32, i32) = (400, 900);
    const POS_9: (i32, i32) = (0, 500);

    #[test]
    fn no_collision() {
        let dxdy = (-6, 6);
        let mut test = Square::new_shape(BOUNDS, dxdy, W_AND_HEIGHT);

        let new_dxdy = test.collision_handeler(dxdy, POS_5, BOUNDS, W_AND_HEIGHT);
        assert_eq!(new_dxdy.0, dxdy.0);
        assert_eq!(new_dxdy.1, dxdy.1);
    }

    #[test]
    fn top_collision() {
        let right_dxdy = (3, -3);
        let left_dxdy = (-3, -3);

        let mut test = Square::new_shape(BOUNDS, (-1, 1), W_AND_HEIGHT);
        /* collisions in the top left & right corners */

        let new_dxdy = test.collision_handeler(right_dxdy, POS_1, BOUNDS, W_AND_HEIGHT);

        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, 3);

        let mut test = Square::new_shape(BOUNDS, (-1, 1), W_AND_HEIGHT);

        let new_dxdy = test.collision_handeler(left_dxdy, POS_4, BOUNDS, W_AND_HEIGHT);

        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, 3);

        /* collisions at the top of the screen, not in the corners*/

        let new_dxdy = test.collision_handeler(right_dxdy, POS_7, BOUNDS, W_AND_HEIGHT);

        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, 3);

        let new_dxdy = test.collision_handeler(left_dxdy, POS_7, BOUNDS, W_AND_HEIGHT);

        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, 3);
    }

    #[test]
    fn bottom_collision() {
        let right_dxdy = (3, 3);
        let left_dxdy = (-3, 3);

        let mut test = Square::new_shape(BOUNDS, (-1, 1), W_AND_HEIGHT);
        /*collisions at the bottom of screen, not corners */

        let new_dxdy = test.collision_handeler(right_dxdy, POS_8, BOUNDS, W_AND_HEIGHT);

        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, -3);

        let mut test = Square::new_shape(BOUNDS, (-1, 1), W_AND_HEIGHT);

        let new_dxdy = test.collision_handeler(left_dxdy, POS_8, BOUNDS, W_AND_HEIGHT);

        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, -3);

        /*bottom corner collisions */

        let mut test = Square::new_shape(BOUNDS, (-1, 1), W_AND_HEIGHT);

        let new_dxdy = test.collision_handeler(right_dxdy, POS_3, BOUNDS, W_AND_HEIGHT);
        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, -3);

        let mut test = Square::new_shape(BOUNDS, left_dxdy, W_AND_HEIGHT);

        let new_dxdy = test.collision_handeler(right_dxdy, POS_2, BOUNDS, W_AND_HEIGHT);

        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, -3);
    }

    #[test]
    fn right_side_collision() {
        let dxdy_up = (3, -3);
        let dxdy_down = (3, 3);

        let mut test = Square::new_shape(BOUNDS, (-1, 1), W_AND_HEIGHT);

        let new_dxdy = test.collision_handeler(dxdy_up, POS_6, BOUNDS, W_AND_HEIGHT);
        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, -3);

        let new_dxdy = test.collision_handeler(dxdy_down, POS_6, BOUNDS, W_AND_HEIGHT);
        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, 3);
    }
    #[test]
    fn left_side_collision() {
        let dxdy_up = (-3, -3);
        let dxdy_down = (-3, 3);

        let mut test = Square::new_shape(BOUNDS, (-1, 1), W_AND_HEIGHT);

        let new_dxdy = test.collision_handeler(dxdy_up, POS_9, BOUNDS, W_AND_HEIGHT);
        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, -3);

        let new_dxdy = test.collision_handeler(dxdy_down, POS_9, BOUNDS, W_AND_HEIGHT);
        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, 3);
    }

    //colision boolean test
    const POINT_1: (i32, i32) = (100, 100);
    const POINT_2: (i32, i32) = (101, 101);
    const POINT_3: (i32, i32) = (100, 101);
    const POINT_4: (i32, i32) = (500, 30);
    const POINT_5: (i32, i32) = (945, 924);
    const POINT_6: (i32, i32) = (500, 500);
    const POINT_7: (i32, i32) = (500, 1000);

    #[test]
    fn coll_bool_1() {
        let mut test_square = Square::new_shape((1000, 1000), (-1, 1), (100, 100));
        test_square._change_location(POINT_1);
        assert_eq!(test_square.collision(), true);
    }

    #[test]
    fn coll_bool_2() {
        let mut test_square = Square::new_shape((1000, 1000), (-1, 1), (100, 100));
        test_square._change_location(POINT_2);
        assert_eq!(test_square.collision(), false);
    }

    #[test]
    fn coll_bool_3() {
        let mut test_square = Square::new_shape((1000, 1000), (-1, 1), (100, 100));
        test_square._change_location(POINT_3);
        assert_eq!(test_square.collision(), true);
    }

    #[test]
    fn coll_bool_4() {
        let mut test_square = Square::new_shape((1000, 1000), (-1, 1), (100, 100));
        test_square._change_location(POINT_4);
        assert_eq!(test_square.collision(), true);
    }

    #[test]
    fn coll_bool_5() {
        let mut test_square = Square::new_shape((1000, 1000), (-1, 1), (100, 100));
        test_square._change_location(POINT_5);
        assert_eq!(test_square.collision(), true);
    }

    #[test]
    fn coll_bool_6() {
        let mut test_square = Square::new_shape((1000, 1000), (-1, 1), (100, 100));
        test_square._change_location(POINT_6);
        assert_eq!(test_square.collision(), false);
    }

    #[test]
    fn coll_bool_7() {
        let mut test_square = Square::new_shape((1000, 1000), (-1, 1), (100, 100));
        test_square._change_location(POINT_7);
        assert_eq!(test_square.collision(), true);
    }
}
