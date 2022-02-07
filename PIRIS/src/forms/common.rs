use iced::{Column, Row};

pub fn _column<'a, M>() -> Column<'a, M> {
    Column::new().spacing(20).padding(10)
}

pub fn row<'a, M>() -> Row<'a, M> {
    Row::new().spacing(20).padding(10)
}
