use rand::Rng;
use sdl2::pixels::Color;

pub fn rng(limits: (i32, i32)) -> (i32, i32) {
    (
        rand::thread_rng().gen_range(0..limits.0),
        rand::thread_rng().gen_range(0..limits.1),
    )
}

/*
    Similar to the other rng function but for direction, and one or both entries in the tupal can be negative
*/
pub fn rng_direction(limits: (i32, i32)) -> (i32, i32) {
    (
        rand::thread_rng().gen_range(limits.0..limits.1),
        rand::thread_rng().gen_range(limits.0..limits.1),
    )
}

pub fn rng_but_color() -> Color {
    Color::RGB(
        get_random_number(),
        get_random_number(),
        get_random_number(),
    )
}

fn get_random_number() -> u8 {
    rand::thread_rng().gen_range(0..255)
}

#[cfg(test)]
mod tests {

    use sdl2::pixels::Color;

    use crate::random_data::{rng, rng_but_color, rng_direction};

    #[test]
    fn test_rng() {
        let limit = (10, 10);

        let random_numbers = rng(limit);

        assert!(random_numbers.0 >= 0 && random_numbers.1 <= 10);

        //test this several more times

        let random_numbers = rng(limit);

        assert!(random_numbers.0 >= 0 && random_numbers.1 <= 10);

        let random_numbers = rng(limit);

        assert!(random_numbers.0 >= 0 && random_numbers.1 <= 10);

        let random_numbers = rng(limit);

        assert!(random_numbers.0 >= 0 && random_numbers.1 <= 10);
    }

    #[test]
    fn test_rng_direction() {
        let limits = (-3, 3);

        let random_dxdy = rng_direction(limits);

        assert!(
            (random_dxdy.0 <= 3 && random_dxdy.0 >= -3)
                && (random_dxdy.1 <= 3 && random_dxdy.1 >= -3)
        );
    }

    #[test]
    fn test_color_gen() {
        let color: Color = rng_but_color();
    }
}
