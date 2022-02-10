use crate::forms::{
    add_user::{AddUser, AddUserMessage},
    common::button,
    deposit::{DepositsMain, DepositsMainMessage},
    list_users::{UserList, UserListMessage},
};
use iced::{button, Column, Container, Element, Sandbox, Text};
use postgres::{Client, NoTls};

pub struct Application {
    client: Client,
    pages: Pages,
}

enum Pages {
    Main {
        add_user_button: button::State,
        list_users_button: button::State,
        deposits_main_button: button::State,
    },
    AddUser(AddUser),
    UserList(UserList),
    DepositsMain(DepositsMain),
}

impl Default for Pages {
    fn default() -> Self {
        Pages::Main {
            add_user_button: button::State::new(),
            list_users_button: button::State::new(),
            deposits_main_button: button::State::new(),
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
    // Application messages
    AddUserButtonPressed,
    UserListButtonPressed,
    DepositsMainButtonPressed,

    // Routing messages
    AddUserMessage(AddUserMessage),
    UserListMessage(UserListMessage),
    DepositsMainMessage(DepositsMainMessage),
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
            pages: Pages::default(),
        }
    }

    fn title(&self) -> String {
        let string = match self.pages {
            Pages::Main { .. } => "Bank Application",
            Pages::AddUser { .. } => "Add new user",
            Pages::UserList { .. } => "View users",
            Pages::DepositsMain { .. } => "Deposits work",
        };
        String::from(string)
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            // Application messages
            ApplicationMessage::AddUserButtonPressed => self.pages = Pages::AddUser(AddUser::new()),
            ApplicationMessage::UserListButtonPressed => {
                self.pages = Pages::UserList(UserList::new(&mut self.client))
            }
            ApplicationMessage::DepositsMainButtonPressed => {
                self.pages = Pages::DepositsMain(DepositsMain::default())
            }

            // Routing messages
            ApplicationMessage::UserListMessage(list_users_message) => {
                match self.pages {
                    Pages::UserList(ref mut list_users) => {
                        let finished = list_users.update(list_users_message, &mut self.client);
                        if finished {
                            self.pages = Pages::default()
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
                            self.pages = Pages::default()
                        }
                    }
                    _ => unreachable!(),
                };
            }
            ApplicationMessage::DepositsMainMessage(deposits_main_message) => {
                match self.pages {
                    Pages::DepositsMain(ref mut add_user) => {
                        let finished = add_user.update(deposits_main_message, &mut self.client);
                        if finished {
                            self.pages = Pages::default()
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let canvass = match &mut self.pages {
            // Application messages
            Pages::Main {
                add_user_button,
                list_users_button,
                deposits_main_button,
            } => Application::column("Bank Application")
                .push(button(
                    "Добавить пользователя",
                    ApplicationMessage::AddUserButtonPressed,
                    add_user_button,
                ))
                .push(button(
                    "Просмотреть пользователей",
                    ApplicationMessage::UserListButtonPressed,
                    list_users_button,
                ))
                .push(button(
                    "Работа с депозитами",
                    ApplicationMessage::DepositsMainButtonPressed,
                    deposits_main_button,
                ))
                .into(),

            // Routing messages
            Pages::AddUser(add_user) => add_user.view().map(ApplicationMessage::AddUserMessage),
            Pages::UserList(list_users) => {
                list_users.view().map(ApplicationMessage::UserListMessage)
            }
            Pages::DepositsMain(deposits_main) => deposits_main
                .view(&mut self.client)
                .map(ApplicationMessage::DepositsMainMessage),
        };
        Container::new(canvass).into()
    }
}
