mod random_data;
mod shape;
//mod random_data;
mod app;
use app::run;
use std::io::Error;
pub fn main() -> Result<(), Error> {
    run().unwrap_or_else(|error| {
        println!("{}", error);
    }); //start the app
    Ok(())
}
