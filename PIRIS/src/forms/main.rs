use crate::forms::{
    add_user::{AddUser, AddUserMessage},
    list_users::{UserList, UserListMessage},
};
use iced::{button, Button, Column, Container, Element, HorizontalAlignment, Sandbox, Text};
use postgres::{Client, NoTls};

pub struct Application {
    client: Client,
    pages: Pages,
}

enum Pages {
    Main {
        add_user_button: button::State,
        list_users_button: button::State,
    },
    AddUser(AddUser),
    UserList(UserList),
}
impl Pages {
    fn new_main() -> Pages {
        Pages::Main {
            add_user_button: button::State::new(),
            list_users_button: button::State::new(),
        }
    }
}

impl Application {
    fn column<'a>(title: &str) -> Column<'a, ApplicationMessage> {
        Column::new()
            .spacing(20)
            .padding(10)
            .push(Text::new(title).size(50))
    }
}

#[derive(Clone, Debug)]
pub enum ApplicationMessage {
    AddUserMessage(AddUserMessage),
    UserListMessage(UserListMessage),
    AddUserButtonPressed,
    UserListButtonPressed,
}

impl Sandbox for Application {
    type Message = ApplicationMessage;

    fn new() -> Self {
        let mut client =
            Client::connect(include_str!("../../sql/rust_sql_config.txt"), NoTls).unwrap();
        client
            .batch_execute(include_str!("../../sql/create_tables.sql"))
            .unwrap();
        client
            .batch_execute(include_str!("../../sql/initialize_tables.sql"))
            .unwrap();

        Application {
            client,
            pages: Pages::Main {
                add_user_button: button::State::new(),
                list_users_button: button::State::new(),
            },
        }
    }

    fn title(&self) -> String {
        let string = match self.pages {
            Pages::Main { .. } => "Bank Application",
            Pages::AddUser { .. } => "Add new user",
            Pages::UserList { .. } => "View users",
        };
        String::from(string)
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ApplicationMessage::AddUserButtonPressed => self.pages = Pages::AddUser(AddUser::new()),
            ApplicationMessage::UserListButtonPressed => {
                self.pages = Pages::UserList(UserList::new(&mut self.client))
            }
            ApplicationMessage::UserListMessage(list_users_message) => {
                match self.pages {
                    Pages::UserList(ref mut list_users) => {
                        let finished = list_users.update(list_users_message, &mut self.client);
                        if finished {
                            self.pages = Pages::new_main()
                        }
                    }
                    _ => unreachable!(),
                };
            }
            ApplicationMessage::AddUserMessage(add_user_message) => {
                match self.pages {
                    Pages::AddUser(ref mut add_user) => {
                        let finished = add_user.update(add_user_message, &mut self.client);
                        if finished {
                            self.pages = Pages::new_main()
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let canvass = match &mut self.pages {
            Pages::Main {
                add_user_button,
                list_users_button,
            } => Application::column("Bank Application")
                .push(
                    button(add_user_button, "Добавить пользователя")
                        .on_press(ApplicationMessage::AddUserButtonPressed),
                )
                .push(
                    button(list_users_button, "Просмотреть пользователей")
                        .on_press(ApplicationMessage::UserListButtonPressed),
                )
                .into(),
            Pages::AddUser(add_user) => add_user.view().map(ApplicationMessage::AddUserMessage),
            Pages::UserList(list_users) => {
                list_users.view().map(ApplicationMessage::UserListMessage)
            }
        };
        Container::new(canvass).into()
    }
}

fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}
