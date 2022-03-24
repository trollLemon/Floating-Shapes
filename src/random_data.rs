pub mod rng {
    use piet::Color;
    use rand::random;

    pub fn rng(limits: (i32, i32)) -> (i32, i32) {
        (
            rand::thread_rng().gen_range(limits[0]..limits[1]),
            rand::thread_rng().gen_range(limits[0]..limits[1]),
        )
    }

    pub fn rng_but_color() -> Color {
        Color::rgb(
            get_random_number(),
            get_random_number(),
            get_random_number(),
        )
    }

    fn get_random_number() -> i32 {
        rand::thread_rng().gen_range(0..255)
    }
}

#[cfg(test)]
mod tests {}
