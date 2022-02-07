use crate::{
    forms::{self, common::row, user::UserMessage},
    state,
};
use iced::{button, Button, Column, Element, Text};
use postgres::{self, Client};

pub struct ModifyUser {
    pub id: i32,
    pub user: forms::user::User,
    pub modify_user_button: button::State,
    pub delete_user_button: button::State,
    pub back_button: button::State,
    pub is_completed: bool,
}

#[derive(Debug, Clone)]
pub enum ModifyUserMessage {
    UserMessage(UserMessage),
    ModifyUserClicked,
    DeleteUserClicked,
    BackClicked,
}

impl ModifyUser {
    pub fn new(id: i32, user: forms::user::User) -> Self {
        ModifyUser {
            id,
            user,
            modify_user_button: button::State::new(),
            delete_user_button: button::State::new(),
            back_button: button::State::new(),
            is_completed: false,
        }
    }
    pub fn view(&mut self) -> Element<ModifyUserMessage> {
        let mut row = row()
            .push(Text::new("Изменить пользователя").size(40))
            .push(
                Button::new(&mut self.back_button, Text::new("Назад"))
                    .on_press(ModifyUserMessage::BackClicked)
                    .padding(10),
            );

        row = if !self.is_completed {
            row.push(
                Button::new(
                    &mut self.modify_user_button,
                    Text::new("Изменить пользователя"),
                )
                .on_press(ModifyUserMessage::ModifyUserClicked)
                .padding(10),
            )
            .push(
                Button::new(
                    &mut self.delete_user_button,
                    Text::new("Удалить пользователя"),
                )
                .on_press(ModifyUserMessage::DeleteUserClicked)
                .padding(10),
            )
        } else {
            row.push(Text::new("Пользователь был успешно отредактирован!").size(20))
        };

        let user = self.user.view().map(ModifyUserMessage::UserMessage);
        Column::new().push(row).push(user).into()
    }
    pub fn update(&mut self, message: ModifyUserMessage, client: &mut Client) -> bool {
        let mut is_finished = false;
        match message {
            ModifyUserMessage::UserMessage(user_message) => self.user.update(user_message),
            ModifyUserMessage::BackClicked => is_finished = true,
            ModifyUserMessage::DeleteUserClicked => {
                self.is_completed = state::user::User::delete(self.id, client).is_ok()
            }
            ModifyUserMessage::ModifyUserClicked => {
                let failure_ids = self.user.verify_static();
                if failure_ids.is_empty() {
                    let result = state::user::User::from_form(&self.user).update(self.id, client);
                    if let Err(ref error) = result {
                        let errors = forms::user::User::verify_dynamic(error);
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
