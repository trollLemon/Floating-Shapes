pub mod collision_handelers {

    //TODO:replace the if else ladder with a different thing
    pub fn collision_handeler(
        current_dxdy: (i32, i32),
        position: (i32, i32),
        bounds: (i32, i32),
    ) -> (i32, i32) {
        //top left corner collision
        if position.0 == 0 && position.1 == 0 {
            (current_dxdy.0 * -1, current_dxdy.1 * -1)
        }
        //top right corner collision
        else if position.0 == bounds.0 && position.1 == 0 {
            (current_dxdy.0 * -1, current_dxdy.1 * -1)
        }
        //top of the screen collision
        else if position.0 < bounds.0 && position.1 == 0 {
            (current_dxdy.0, current_dxdy.1 * -1)
        }
        //bottom left corner collision
        else if position.0 == 0 && position.1 == bounds.1 {
            (current_dxdy.0 * -1, current_dxdy.1 * -1)
        }
        //bottom right corner collision
        else if position.0 == bounds.0 && position.1 == bounds.1 {
            (current_dxdy.0 * -1, current_dxdy.1 * -1)
        }
        //bottom collision
        else if position.0 < bounds.0 && position.1 == bounds.1 {
            (current_dxdy.0, current_dxdy.1 * -1)
        }
        //right side collision
        else if position.0 == bounds.0 && position.1 < bounds.1 {
            (current_dxdy.0 * -1, current_dxdy.1)
        }
        //left side collision
        else if position.0 == 0 && position.1 < bounds.1 {
            (current_dxdy.0 * -1, current_dxdy.1)
        }
        //No collision: the function should return the same dxdy since its not hitting anything
        else {
            current_dxdy
        }
    }
}

#[cfg(test)]
mod tests {

    use super::collision_handelers::*;

    const BOUNDS: (i32, i32) = (1000, 1000);

    const POS_1: (i32, i32) = (1000, 0);
    const POS_2: (i32, i32) = (0, 1000);
    const POS_3: (i32, i32) = (1000, 1000);
    const POS_4: (i32, i32) = (0, 0);
    const POS_5: (i32, i32) = (100, 500);
    const POS_6: (i32, i32) = (1000, 500);
    const POS_7: (i32, i32) = (500, 0);
    const POS_8: (i32, i32) = (400, 1000);
    const POS_9: (i32, i32) = (0, 500);

    #[test]
    fn no_collision() {
        let dxdy = (-6, 6);
        let new_dxdy = collision_handeler(dxdy, POS_5, BOUNDS);
        assert_eq!(new_dxdy.0, dxdy.0);
        assert_eq!(new_dxdy.1, dxdy.1);
    }

    #[test]
    fn top_collision() {
        let right_dxdy = (3, -3);
        let left_dxdy = (-3, -3);

        /* collisions in the top left & right corners */

        let new_dxdy = collision_handeler(right_dxdy, POS_1, BOUNDS);

        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, 3);

        let new_dxdy = collision_handeler(left_dxdy, POS_4, BOUNDS);

        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, 3);

        /* collisions at the top of the screen, not in the corners*/

        let new_dxdy = collision_handeler(right_dxdy, POS_7, BOUNDS);

        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, 3);

        let new_dxdy = collision_handeler(left_dxdy, POS_7, BOUNDS);

        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, 3);
    }

    #[test]
    fn bottom_collision() {
        let right_dxdy = (3, 3);
        let left_dxdy = (-3, 3);

        /*collisions at the bottom of screen, not corners */

        let new_dxdy = collision_handeler(right_dxdy, POS_8, BOUNDS);

        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, -3);

        let new_dxdy = collision_handeler(left_dxdy, POS_8, BOUNDS);

        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, -3);

        /*bottom corner collisions */

        let new_dxdy = collision_handeler(right_dxdy, POS_3, BOUNDS);
        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, -3);

        let new_dxdy = collision_handeler(right_dxdy, POS_2, BOUNDS);

        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, -3);
    }

    #[test]
    fn right_side_collision() {
        let dxdy_up = (3, -3);
        let dxdy_down = (3, 3);

        let new_dxdy = collision_handeler(dxdy_up, POS_6, BOUNDS);
        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, -3);

        let new_dxdy = collision_handeler(dxdy_down, POS_6, BOUNDS);
        assert_eq!(new_dxdy.0, -3);
        assert_eq!(new_dxdy.1, 3);
    }
    #[test]
    fn left_side_collision() {
        let dxdy_up = (-3, -3);
        let dxdy_down = (-3, 3);

        let new_dxdy = collision_handeler(dxdy_up, POS_9, BOUNDS);
        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, -3);

        let new_dxdy = collision_handeler(dxdy_down, POS_9, BOUNDS);
        assert_eq!(new_dxdy.0, 3);
        assert_eq!(new_dxdy.1, 3);
    }
}
