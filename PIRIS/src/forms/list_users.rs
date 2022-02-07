use crate::forms::modify_user::{ModifyUser, ModifyUserMessage};
use crate::forms::{self, common::row};
use crate::state::user::User;
use iced::{button, Button, Column, Element, Length, Row, Text};
use postgres::{self, Client};

pub enum UserList {
    /// When this form is present, show it instead
    ModifyUser(ModifyUser),
    UserList {
        listed_users: Vec<(ListedUser, button::State)>,
        to_main_button: button::State,
    },
}

#[derive(Debug, Clone)]
pub enum UserListMessage {
    ModifyUser(ModifyUserMessage),
    UpdateUserClicked(usize),
    ToMainClicked,
}

pub struct ListedUser {
    id: i32,
    name: String,
    surname: String,
    father_name: String,
    passport_idx_number: String,
}

impl ListedUser {
    pub fn from_row(row: postgres::Row) -> ListedUser {
        let id = row.get(0);
        let name = row.get(1);
        let surname = row.get(2);
        let father_name = row.get(3);
        let passport_idx_number = row.get(4);

        ListedUser {
            id,
            name,
            surname,
            father_name,
            passport_idx_number,
        }
    }
}

impl UserList {
    pub fn new(client: &mut Client) -> Self {
        let mut listed_users = Vec::new();
        let rows = client
            .query(include_str!("../../sql/select_listed_user.sql"), &[])
            .unwrap();
        for row in rows {
            listed_users.push(ListedUser::from_row(row))
        }

        let listed_users = listed_users
            .into_iter()
            .map(|user| (user, button::State::new()))
            .collect();

        UserList::UserList {
            listed_users,
            to_main_button: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<UserListMessage> {
        match self {
            UserList::ModifyUser(modify_user) => {
                modify_user.view().map(UserListMessage::ModifyUser)
            }
            UserList::UserList {
                listed_users,
                to_main_button,
            } => {
                let mut no = 0;
                let mut column = Column::new().push(
                    row().push(Text::new("Список пользователей").size(40)).push(
                        Button::new(to_main_button, Text::new("Назад"))
                            .on_press(UserListMessage::ToMainClicked)
                            .padding(10),
                    ),
                );
                for (listed_user, button) in listed_users {
                    column = column.push(
                        Row::new()
                            .spacing(5)
                            .push(Text::new(listed_user.surname.clone()).width(Length::Units(200)))
                            .push(Text::new(listed_user.name.clone()).width(Length::Units(200)))
                            .push(
                                Text::new(listed_user.father_name.clone())
                                    .width(Length::Units(200)),
                            )
                            .push(
                                Text::new(listed_user.passport_idx_number.clone())
                                    .width(Length::Units(200)),
                            )
                            .push(
                                Button::new(button, Text::new("Редактировать"))
                                    .on_press(UserListMessage::UpdateUserClicked(no)),
                            ),
                    );
                    no += 1;
                }

                if no == 0 {
                    column = column.push(Text::new("Ничего не найдено!").size(30))
                }
                column.into()
            }
        }
    }

    pub fn update(&mut self, message: UserListMessage, client: &mut Client) -> bool {
        let mut is_finished = false;
        match message {
            UserListMessage::ToMainClicked => is_finished = true,
            UserListMessage::UpdateUserClicked(no) => match self {
                UserList::UserList { listed_users, .. } => {
                    let id = listed_users[no].0.id;
                    let row = client
                        .query_one(include_str!("../../sql/select_whole_user.sql"), &[&id])
                        .unwrap();
                    let user = User::from_row(row);
                    let user = forms::user::User::from_state(user);
                    *self = UserList::ModifyUser(ModifyUser::new(id, user));
                }
                _ => unreachable!(),
            },
            UserListMessage::ModifyUser(modify_user_message) => match self {
                UserList::ModifyUser(modify_user) => {
                    let is_finished = modify_user.update(modify_user_message, client);
                    if is_finished {
                        *self = UserList::new(client);
                    }
                }
                _ => unreachable!(),
            },
        }
        is_finished
    }
}
