use rand::Rng;
use sdl2::pixels::Color;

pub fn rng(limits: (i32, i32)) -> (i32, i32) {
    (
        rand::thread_rng().gen_range(0..limits.1),
        rand::thread_rng().gen_range(0..limits.1),
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

    use crate::random_data::{rng, rng_but_color};

    #[test]
    fn test_rng() {
        let limit = (10, 10);

        let random_numbers = rng(limit);

        println!("{},{}", limit.0, limit.1);

        assert_eq!(random_numbers.0 >= 0 && random_numbers.1 <= 10, true);

        //test this several more times

        let random_numbers = rng(limit);

        assert_eq!(random_numbers.0 >= 0 && random_numbers.1 <= 10, true);

        let random_numbers = rng(limit);

        assert_eq!(random_numbers.0 >= 0 && random_numbers.1 <= 10, true);

        let random_numbers = rng(limit);

        assert_eq!(random_numbers.0 >= 0 && random_numbers.1 <= 10, true);
    }

    #[test]
    fn test_color_gen() {
        let color: Color = rng_but_color();
    }
}
