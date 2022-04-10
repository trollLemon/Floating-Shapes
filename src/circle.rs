pub mod circle {

    use crate::random_data::{rng, rng_but_color, rng_direction};
    use crate::shape::shapes::Shape;
    use sdl2::pixels::Color;

    pub struct Circle {
        color: Color,
        radius: i32,
        xy: (i32, i32),
        dxdy: (i32, i32),
        bounds: (i32, i32),
    }

    impl Shape for Circle {
        fn new_shape(limits: (i32, i32), delta: (i32, i32), dimensions: (i32, i32)) -> Self {
            Self {
                color: rng_but_color(),
                radius: dimensions.0,
                xy: rng((limits.0 - dimensions.0 * 2, limits.1 - dimensions.1 * 2)),
                dxdy: rng_direction(delta),
                bounds: limits,
            }
        }

        fn update_position(&mut self) {
            self.xy = (self.xy.0 + self.dxdy.0, self.xy.1 + self.dxdy.1);
        }

        fn change_direction(&mut self) {
            if self.is_collidable() {
                self.dxdy = self.collision_handeler(self.dxdy, self.xy, self.bounds, self.radius);
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

    impl Circle {
        pub fn collision_handeler(
            &mut self,
            current_dxdy: (i32, i32),
            position: (i32, i32),
            bounds: (i32, i32),
            radius: i32,
        ) -> (i32, i32) {
            //top left corner collision
            if position.0 - radius <= 0 && position.1 - radius <= 0 {
                (current_dxdy.0 * -1, current_dxdy.1 * -1)
            }
            //top right corner collision
            else if position.0 >= bounds.0 - radius && position.1 <= 0 {
                (current_dxdy.0 * -1, current_dxdy.1 * -1)
            }
            //top of the screen collision
            else if position.0 < bounds.0 && position.1 - radius <= 0 {
                (current_dxdy.0, current_dxdy.1 * -1)
            }
            //bottom left corner collision
            else if position.0 <= 0 && position.1 >= bounds.1 {
                (current_dxdy.0 * -1, current_dxdy.1 * -1)
            }
            //bottom right corner collision
            else if position.0 >= bounds.0 - radius && position.1 >= bounds.1 - radius {
                (current_dxdy.0 * -1, current_dxdy.1 * -1)
            }
            //bottom collision
            else if position.0 + radius < bounds.0 && position.1 >= bounds.1 {
                (current_dxdy.0, current_dxdy.1 * -1)
            }
            //right side collision
            else if position.0 >= bounds.0 && position.1 < bounds.1 {
                (current_dxdy.0 * -1, current_dxdy.1)
            }
            //left side collision
            else if position.0 - radius <= 0 && position.1 < bounds.1 {
                (current_dxdy.0 * -1, current_dxdy.1)
            }
            //No collision: the function should return the same dxdy since its not hitting anything
            else {
                current_dxdy
            }
        }

        pub fn is_collidable(&mut self) -> bool {
            //if the coordinates of the shape are near the dimensions, then it may collide soon, so return true

            if (self.get_location().0 > self.radius && self.get_location().1 > self.radius)
                && (self.get_location().0 < self.bounds.0 - self.radius
                    && self.get_location().1 < self.bounds.1 - self.radius)
            {
                false
            } else {
                true
            }
        }

        pub fn get_radius(&mut self) -> i32 {
            self.radius
        }
    }
}

#[cfg(test)]
mod tests {

    //collision tests

    use crate::shape::shapes::Shape;

    use super::circle::Circle;

    const BOUNDS: (i32, i32) = (1000, 1000);

    const RADIUS: i32 = 50;

    const POS_1: (i32, i32) = (950, 0);
    const POS_2: (i32, i32) = (0, 1000);
    const POS_3: (i32, i32) = (950, 950);
    const POS_4: (i32, i32) = (50, 50);
    const POS_5: (i32, i32) = (100, 500);
    const POS_6: (i32, i32) = (950, 500);
    const POS_7: (i32, i32) = (500, 50);
    const POS_8: (i32, i32) = (400, 950);
    const POS_9: (i32, i32) = (0, 500);

    #[test]
    fn top_right_collision() {
        let mut test = Circle::new_shape(BOUNDS, (-1, 1), (RADIUS, RADIUS));

        let dydx = (4, -4);
        let new_dydx = test.collision_handeler(dydx, POS_1, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, -4);
        assert_eq!(new_dydx.1, 4);
    }

    #[test]
    fn bottom_left_collision() {
        let mut test = Circle::new_shape(BOUNDS, (-1, 1), (RADIUS, RADIUS));
        let dydx = (-4, 4);
        let new_dydx = test.collision_handeler(dydx, POS_2, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, 4);
        assert_eq!(new_dydx.1, -4);
    }

    #[test]
    fn bottom_right_collision() {
        let mut test = Circle::new_shape(BOUNDS, (-1, 1), (RADIUS, RADIUS));
        let dydx = (4, 4);
        let new_dydx = test.collision_handeler(dydx, POS_3, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, -4);
        assert_eq!(new_dydx.1, -4);
    }

    #[test]
    fn top_left_collision() {
        let mut test = Circle::new_shape(BOUNDS, (-1, 1), (RADIUS, RADIUS));
        let dydx = (-4, -4);
        let new_dydx = test.collision_handeler(dydx, POS_4, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, 4);
        assert_eq!(new_dydx.1, 4);
    }

    #[test]
    fn no_collision() {
        let mut test = Circle::new_shape(BOUNDS, (-1, 1), (RADIUS, RADIUS));
        let dydx = (-4, 4);
        let new_dydx = test.collision_handeler(dydx, POS_5, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, dydx.0);
        assert_eq!(new_dydx.1, dydx.1);
    }

    #[test]
    fn right_collision() {
        let mut test = Circle::new_shape(BOUNDS, (-1, 1), (RADIUS, RADIUS));
        let up_dydx = (4, -4);
        let down_dydx = (4, 4);
        let new_dydx = test.collision_handeler(up_dydx, POS_6, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, -4);
        assert_eq!(new_dydx.1, -4);

        let new_dydx = test.collision_handeler(down_dydx, POS_6, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, -4);
        assert_eq!(new_dydx.1, 4);
    }

    #[test]
    fn top_collision() {
        let mut test = Circle::new_shape(BOUNDS, (-1, 1), (RADIUS, RADIUS));
        let right_dydx = (4, -4);
        let left_dydx = (-4, -4);
        let new_dydx = test.collision_handeler(right_dydx, POS_7, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, 4);
        assert_eq!(new_dydx.1, 4);

        let new_dydx = test.collision_handeler(left_dydx, POS_7, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, -4);
        assert_eq!(new_dydx.1, 4);
    }

    #[test]
    fn left_collision() {
        let mut test = Circle::new_shape(BOUNDS, (-1, 1), (RADIUS, RADIUS));
        let up_dydx = (-4, -4);
        let down_dydx = (-4, 4);
        let new_dydx = test.collision_handeler(up_dydx, POS_9, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, 4);
        assert_eq!(new_dydx.1, -4);

        let new_dydx = test.collision_handeler(down_dydx, POS_9, BOUNDS, RADIUS);
        assert_eq!(new_dydx.0, 4);
        assert_eq!(new_dydx.1, 4);
    }

    #[test]
    fn bottom_collision() {
        let mut test = Circle::new_shape(BOUNDS, (-1, 1), (RADIUS, RADIUS));
        let right_dxdy = (4, 4);
        let left_dxdy = (-4, 4);

        let new_dxdy = test.collision_handeler(right_dxdy, POS_8, BOUNDS, RADIUS);

        assert_eq!(new_dxdy.0, 4);
        assert_eq!(new_dxdy.1, -4);

        let new_dxdy = test.collision_handeler(left_dxdy, POS_8, BOUNDS, RADIUS);

        assert_eq!(new_dxdy.0, -4);
        assert_eq!(new_dxdy.1, -4);
    }
}
