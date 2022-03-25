mod random_data;
mod shape;

mod app;
use app::run;
use std::io::Error;
pub fn main() -> Result<(), Error> {
    run().unwrap_or_else(|error| {
        println!("{}", error);
    }); //start the app, output an error if it crashes
    Ok(())
}
