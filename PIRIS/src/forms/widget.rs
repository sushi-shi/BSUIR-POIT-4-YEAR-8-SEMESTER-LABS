use crate::state::validation::Validation;
use iced::{
    button, pick_list, text_input, Button, Checkbox, Column, Element, Length, PickList, Radio,
    Text, TextInput,
};
use iced_aw::date_picker::{self, Date, DatePicker};

pub enum Widget {
    Text {
        value: String,
        validation: Validation,
        text_input: text_input::State,
    },
    Radio {
        value: usize,
        possible_values: Vec<String>,
    },
    Checkbox {
        value: String,
        is_checked: bool,
    },
    Pick {
        value: Option<String>,
        possible_values: Vec<String>,
        pick_list: pick_list::State<String>,
    },
    Date {
        value: Option<Date>,
        date_picker: date_picker::State,
        button: button::State,
    },
}

#[derive(Debug, Clone)]
pub enum WidgetMessage {
    TextChanged(String),
    RadioSelected(usize),
    PickPicked(String),
    DatePicked(DateMessage),
    CheckboxToggled(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum DateMessage {
    Cancel,
    Submit(Date),
    Choose,
}

impl Widget {
    pub fn text(&self) -> String {
        match self {
            Widget::Text { value, .. } => value.clone(),
            _ => panic!(),
        }
    }

    pub fn radio(&self) -> (usize, Vec<String>) {
        match self {
            Widget::Radio {
                value,
                possible_values,
                ..
            } => (*value, possible_values.clone()),
            _ => panic!(),
        }
    }

    pub fn pick(&self) -> (Option<String>, Vec<String>) {
        match self {
            Widget::Pick {
                value,
                possible_values,
                ..
            } => (value.clone(), possible_values.clone()),
            _ => panic!(),
        }
    }

    pub fn date(&self) -> Option<Date> {
        match self {
            Widget::Date { value, .. } => *value,
            _ => panic!(),
        }
    }

    pub fn checkbox(&self) -> bool {
        match self {
            Widget::Checkbox { is_checked, .. } => *is_checked,
            _ => panic!(),
        }
    }

    pub fn new_text(value: String, validation: Validation) -> Widget {
        Widget::Text {
            value,
            validation,
            text_input: text_input::State::new(),
        }
    }

    pub fn new_radio(value: usize, possible_values: Vec<String>) -> Widget {
        Widget::Radio {
            value,
            possible_values,
        }
    }

    pub fn new_pick(value: Option<String>, possible_values: Vec<String>) -> Widget {
        Widget::Pick {
            value,
            possible_values,
            pick_list: pick_list::State::default(),
        }
    }

    pub fn new_date(value: Option<Date>) -> Widget {
        Widget::Date {
            value,
            date_picker: date_picker::State::now(),
            button: button::State::new(),
        }
    }

    pub fn new_checkbox(value: String, is_checked: bool) -> Widget {
        Widget::Checkbox { value, is_checked }
    }

    pub fn is_correct(&self, is_required: bool) -> bool {
        match self {
            Widget::Text {
                validation, value, ..
            } => {
                let is_empty = value.trim().is_empty();
                validation(value) || (!is_required && is_empty)
            }
            Widget::Radio { .. } | Widget::Checkbox { .. } => true,
            Widget::Pick { value, .. } => !is_required || value.is_some(),
            Widget::Date { value, .. } => !is_required || value.is_some(),
        }
    }

    pub fn view(&mut self) -> Element<WidgetMessage> {
        let units = match self {
            Widget::Date { .. } | Widget::Checkbox { .. } => 150,
            _ => 200,
        };
        let mut column = Column::new().width(Length::Units(units));
        match self {
            Widget::Text {
                value, text_input, ..
            } => {
                column = column.push(TextInput::new(
                    text_input,
                    "",
                    &*value,
                    WidgetMessage::TextChanged,
                ));
            }
            Widget::Radio {
                value,
                possible_values,
            } => {
                for (i, possible_value) in possible_values.iter().enumerate() {
                    column = column.push(Radio::new(
                        i,
                        possible_value.clone(),
                        Some(*value),
                        WidgetMessage::RadioSelected,
                    ));
                }
            }
            Widget::Pick {
                pick_list,
                possible_values,
                value,
            } => {
                column = column.push(PickList::new(
                    pick_list,
                    possible_values.clone(),
                    value.clone(),
                    WidgetMessage::PickPicked,
                ))
            }
            Widget::Date {
                date_picker,
                button,
                value,
            } => {
                let button = Button::new(button, Text::new("Выбрать дату"))
                    .on_press(WidgetMessage::DatePicked(DateMessage::Choose));

                column = column.push(DatePicker::new(
                    date_picker,
                    button,
                    WidgetMessage::DatePicked(DateMessage::Cancel),
                    move |new_value| WidgetMessage::DatePicked(DateMessage::Submit(new_value)),
                ));
                if let Some(value) = value {
                    column = column.push(Text::new(format!("Дата: {}", value)));
                }
            }
            Widget::Checkbox { value, is_checked } => {
                column = column.push(Checkbox::new(
                    *is_checked,
                    value.clone(),
                    WidgetMessage::CheckboxToggled,
                ))
            }
        }
        column.into()
    }

    pub fn update(&mut self, message: WidgetMessage) {
        match message {
            WidgetMessage::TextChanged(new_value) => match self {
                Widget::Text { ref mut value, .. } => *value = new_value,
                _ => unreachable!(),
            },
            WidgetMessage::RadioSelected(new_value) => match self {
                Widget::Radio { ref mut value, .. } => *value = new_value,
                _ => unreachable!(),
            },
            WidgetMessage::PickPicked(new_value) => match self {
                Widget::Pick { ref mut value, .. } => *value = Some(new_value),
                _ => unreachable!(),
            },
            WidgetMessage::DatePicked(date_message) => match self {
                Widget::Date {
                    ref mut value,
                    ref mut date_picker,
                    ..
                } => match date_message {
                    DateMessage::Choose => {
                        date_picker.reset();
                        date_picker.show(true);
                    }
                    DateMessage::Submit(new_value) => {
                        *value = Some(new_value);
                        date_picker.show(false);
                    }
                    DateMessage::Cancel => {
                        date_picker.show(false);
                    }
                },
                _ => unreachable!(),
            },
            WidgetMessage::CheckboxToggled(new_value) => match self {
                Widget::Checkbox {
                    ref mut is_checked, ..
                } => *is_checked = new_value,
                _ => unreachable!(),
            },
        }
    }
}
