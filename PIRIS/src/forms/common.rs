use iced::{button, Button, Column, Row, Text};

pub fn column<'a, M>() -> Column<'a, M> {
    Column::new().spacing(20).padding(10)
}

pub fn row<'a, M>() -> Row<'a, M> {
    Row::new().spacing(20).padding(10)
}

pub fn button<'a, M: Clone + std::fmt::Debug>(
    label: &str,
    message: M,
    state: &'a mut button::State,
) -> Button<'a, M> {
    Button::new(state, Text::new(label))
        .on_press(message)
        .padding(10)
}

pub fn canvass<'a, M>(name: &str) -> Row<'a, M> {
    Row::new()
        .spacing(10)
        .padding(10)
        .push(Text::new(name).size(20))
}
