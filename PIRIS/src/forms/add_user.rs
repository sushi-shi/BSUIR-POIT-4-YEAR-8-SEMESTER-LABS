use crate::{
    forms::common::row,
    forms::user::{User, UserMessage},
    state,
};
use iced::{button, Button, Column, Element, Text};
use postgres::Client;

pub struct AddUser {
    pub user: User,
    pub add_user_button: button::State,
    pub to_main_button: button::State,
    pub is_completed: bool,
}

#[derive(Debug, Clone)]
pub enum AddUserMessage {
    UserMessage(UserMessage),
    AddUserClicked,
    ToMainClicked,
}

impl AddUser {
    pub fn new() -> AddUser {
        AddUser {
            user: User::new(),
            add_user_button: button::State::new(),
            to_main_button: button::State::new(),
            is_completed: false,
        }
    }

    pub fn view(&mut self) -> Element<AddUserMessage> {
        let mut row = row()
            .push(Text::new("Добавить пользователя").size(40))
            .push(
                Button::new(&mut self.to_main_button, Text::new("Назад"))
                    .on_press(AddUserMessage::ToMainClicked)
                    .padding(10),
            );

        row = if !self.is_completed {
            row.push(
                Button::new(
                    &mut self.add_user_button,
                    Text::new("Добавить пользователя"),
                )
                .on_press(AddUserMessage::AddUserClicked)
                .padding(10),
            )
        } else {
            row.push(Text::new("Пользователь был успешно добавлен!").size(20))
        };

        let user = self.user.view().map(AddUserMessage::UserMessage);
        Column::new().push(row).push(user).into()
    }

    pub fn update(&mut self, message: AddUserMessage, client: &mut Client) -> bool {
        let mut is_finished = false;
        match message {
            AddUserMessage::UserMessage(user_message) => self.user.update(user_message),
            AddUserMessage::ToMainClicked => is_finished = true,
            AddUserMessage::AddUserClicked => {
                let failure_ids = self.user.verify_static();
                if failure_ids.is_empty() {
                    let result = state::user::User::from_form(&self.user).insert(client);
                    if let Err(ref error) = result {
                        let errors = User::verify_dynamic(error);
                        for error_no in errors {
                            self.user.fields[error_no].has_failed = true;
                        }
                    }
                    self.is_completed = result.is_ok();
                } else {
                    for failure_id in failure_ids {
                        self.user.fields[failure_id].has_failed = true;
                    }
                }
            }
        }
        is_finished
    }
}
