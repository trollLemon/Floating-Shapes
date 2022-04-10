pub mod shapes {
    use sdl2::pixels::Color;
    //every shape will have a function to create itself and move itself
    //requires the width and height of the window the shape is drawn on so it doesnt get drawn off the screen
    //Also requires a tupal of the desired change in x and y. This allows for the shapes to have different speeds as they are created
    //functions to change the shapes location, direction and speed are also included
    pub trait Shape {
        fn new_shape(bounds: (i32, i32), delta: (i32, i32), dimensions: (i32, i32)) -> Self;
        fn update_position(&mut self);
        fn change_direction(&mut self);

        fn get_location(&mut self) -> (i32, i32);
        fn get_direction(&mut self) -> (i32, i32);
        fn get_color(&mut self) -> Color;
    }
}
