pub mod ui {

    use druid::widget::{Button, Flex, Label};

    use druid::{LocalizedString, Widget, WidgetExt};

    static DELTA: (i32, i32) = (3, 3); //change in x and y coordinates, can be changed within the app wiith user input

    pub fn ui_builder() -> impl Widget<u32> {
        todo!();
    }
}
