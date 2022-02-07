use crate::forms::widget::{Widget, WidgetMessage};
use iced::{Column, Element, Text};

pub struct Field {
    /// Where in a Vector a field is located
    pub id: usize,
    /// Label to succintly describe a field
    pub label: String,
    /// Whether it is required or not
    pub is_required: bool,
    /// Whether it was verfied and the test failed
    pub has_failed: bool,
    /// An internal widgted of a field
    pub widget: Widget,
}

#[derive(Debug, Clone)]
pub struct FieldMessage {
    pub id: usize,
    pub widget_message: WidgetMessage,
}

impl Field {
    pub fn new(id: usize, label: &str, is_required: bool, widget: Widget) -> Field {
        Field {
            id,
            label: label.to_string(),
            is_required,
            has_failed: false,
            widget,
        }
    }

    pub fn has_failed(&self) -> (usize, bool) {
        (self.id, !self.widget.is_correct(self.is_required))
    }

    pub fn view(&mut self) -> Element<FieldMessage> {
        let label = {
            let mut label = Text::new(&self.label);
            if self.has_failed {
                // RED
                label = label.color([1.0, 0.0, 0.0]);
            }
            label
        };

        let widget = {
            let id = self.id;
            let widget = self
                .widget
                .view()
                .map(move |widget_message| FieldMessage { id, widget_message });
            widget
        };

        Column::new()
            .padding(5)
            .spacing(5)
            .push(label)
            .push(widget)
            .into()
    }

    pub fn update(&mut self, message: FieldMessage) {
        self.widget.update(message.widget_message);
    }
}
