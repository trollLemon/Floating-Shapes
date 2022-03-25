use std::io::Error;

use crate::shape::shapes;

/*
    a bunch of vectors that store each shape so we can keep track of thier position and do collisions
*/
static SQUARE_OBJECTS: Vec<shapes::Square> = vec![];
static CIRCLE_OBJECTS: Vec<shapes::Circle> = vec![];
static TRIANGLE_OBJECTS: Vec<shapes::Triangle> = vec![];
static TRAPAZIOD_OBJECTS: Vec<shapes::Trapaziod> = vec![];

pub fn run() -> Result<(), Error> {
    Ok(())
}
