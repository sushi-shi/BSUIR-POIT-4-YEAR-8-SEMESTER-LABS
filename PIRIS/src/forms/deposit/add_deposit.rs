use crate::{
    forms::common::{self, button, column, row},
    state::{self, utils},
};
use iced::{button, pick_list, text_input, Element, Length, PickList, Row, Text, TextInput};
use postgres::Client;

pub struct AddDeposit {
    pub client: pick_list::State<String>,
    pub deposit_program: pick_list::State<String>,
    pub is_revokable: text_input::State,
    pub start_date: text_input::State,
    pub end_date: text_input::State,
    pub currency: pick_list::State<String>,
    pub duration: pick_list::State<String>,
    pub percentage: text_input::State,
    pub min_amount: text_input::State,

    pub deposited_amount: text_input::State,
    pub deposit_no: text_input::State,

    pub add_deposit_button: button::State,
    pub back_button: button::State,

    pub is_completed: bool,
    pub add_deposit_state: state::AddDepositState,
    pub add_deposit: state::AddDeposit,
}

#[derive(Debug, Clone)]
pub enum AddDepositMessage {
    Client(String),
    DepositProgram(String),

    Currency(String),
    Duration(String),

    DepositedAmount(String),
    DepositedNo(String),

    Readonly,

    AddDepositButton,
    BackButton,
}

const CLIENT: &str = "Выбрать клиента";
const DEPOSIT_PROGRAM: &str = "Выбрать депозит";
const IS_REVOKABLE: &str = "Отзывная";
const DATE: &str = "Доступна c/по";

const HEADER: &str = "Оформление депозита";
const BACK: &str = "Назад";

impl AddDeposit {
    pub fn new(client: &mut Client) -> AddDeposit {
        let add_deposit = state::AddDeposit::build(client);

        AddDeposit {
            client: pick_list::State::default(),
            deposit_program: pick_list::State::default(),
            is_revokable: text_input::State::default(),
            start_date: text_input::State::default(),
            end_date: text_input::State::default(),
            currency: pick_list::State::default(),
            duration: pick_list::State::default(),
            percentage: text_input::State::default(),
            min_amount: text_input::State::default(),

            deposited_amount: text_input::State::default(),
            deposit_no: text_input::State::default(),

            add_deposit_button: button::State::default(),
            back_button: button::State::default(),

            is_completed: false,
            add_deposit_state: state::AddDepositState::default(),
            add_deposit,
        }
    }

    pub fn view(&mut self, client: &mut Client) -> Element<AddDepositMessage> {
        // A column contains multiple rows described below
        column()
            .push(view_header(
                self.is_completed,
                &mut self.back_button,
                &mut self.add_deposit_button,
            ))
            .push(view_client(
                &self.add_deposit,
                &self.add_deposit_state,
                &mut self.client,
            ))
            .push(view_deposit_program(
                &self.add_deposit,
                &self.add_deposit_state,
                &mut self.deposit_program,
                &mut self.is_revokable,
            ))
            .push(view_date(
                &self.add_deposit,
                &self.add_deposit_state,
                &mut self.start_date,
                &mut self.end_date,
            ))
            .into()
    }

    pub fn update(&mut self, message: AddDepositMessage, client: &mut Client) -> bool {
        let mut is_finished = false;
        match message {
            AddDepositMessage::Client(chosen) => {
                self.add_deposit_state.client = Some(chosen);
            }
            AddDepositMessage::DepositProgram(chosen) => {
                self.add_deposit_state.deposit_program = Some(chosen);
            }
            AddDepositMessage::Readonly => (),
            AddDepositMessage::AddDepositButton => {
                // use cllient and store stuff in db
            }
            AddDepositMessage::BackButton => is_finished = true,
            _ => todo!(),
        }
        is_finished
    }
}

fn view_header<'a>(
    is_completed: bool,

    back_button: &'a mut button::State,
    add_deposit_button: &'a mut button::State,
) -> Row<'a, AddDepositMessage> {
    let mut header = row().push(Text::new(HEADER).size(40)).push(button(
        BACK,
        AddDepositMessage::BackButton,
        back_button,
    ));
    header = if !is_completed {
        header.push(button(
            "Оформить",
            AddDepositMessage::AddDepositButton,
            add_deposit_button,
        ))
    } else {
        header.push(Text::new("Депозит успешно оформлен").size(20))
    };
    header
}

fn view_client<'a>(
    add_deposit: &state::AddDeposit,
    add_deposit_state: &state::AddDepositState,

    client: &'a mut pick_list::State<String>,
) -> Row<'a, AddDepositMessage> {
    let canvass = common::canvass(CLIENT);

    let users: Vec<_> = add_deposit.users.iter().map(|x| x.1.clone()).collect();
    let selected_user = add_deposit_state.client.clone();
    canvass.push(PickList::new(
        client,
        users,
        selected_user,
        AddDepositMessage::Client,
    ))
}

fn view_deposit_program<'a>(
    add_deposit: &state::AddDeposit,
    add_deposit_state: &state::AddDepositState,

    deposit_program: &'a mut pick_list::State<String>,
    is_revokable: &'a mut text_input::State,
) -> Row<'a, AddDepositMessage> {
    let mut canvass = common::canvass(DEPOSIT_PROGRAM);

    // 1
    let deposit_programs: Vec<_> = add_deposit
        .deposit_programs
        .iter()
        .map(|x| x.name.clone())
        .collect();
    let selected_deposit_program = add_deposit_state.deposit_program.clone();
    canvass = canvass.push(PickList::new(
        deposit_program,
        deposit_programs,
        selected_deposit_program.clone(),
        AddDepositMessage::DepositProgram,
    ));

    // 2
    let text = match selected_deposit_program {
        Some(deposit_program_name) => {
            let is_revokable = add_deposit
                .deposit_programs
                .iter()
                .find(|&deposit_program| deposit_program.name == deposit_program_name)
                .unwrap()
                .is_revokable;
            if is_revokable {
                "Да"
            } else {
                "Нет"
            }
        }
        None => "",
    };
    canvass.push(Text::new(IS_REVOKABLE).size(20)).push(
        TextInput::new(
            is_revokable,
            "",
            text,
            utils::fst(AddDepositMessage::Readonly),
        )
        .width(Length::Units(40)),
    )
}

fn view_date<'a>(
    add_deposit: &state::AddDeposit,
    add_deposit_state: &state::AddDepositState,

    start_date: &'a mut text_input::State,
    end_date: &'a mut text_input::State,
) -> Row<'a, AddDepositMessage> {
    let canvass = common::canvass(DATE);
    let (start_date_text, end_date_text) = {
        match add_deposit.deposit_program(add_deposit_state) {
            None => ("", ""),
            Some(ref deposit_program) => (
                &deposit_program.start_date[..],
                &deposit_program.end_date[..],
            ),
        }
    };

    canvass
        .push(
            TextInput::new(
                start_date,
                "",
                start_date_text,
                utils::fst(AddDepositMessage::Readonly),
            )
            .width(Length::Units(100)),
        )
        .push(
            TextInput::new(
                end_date,
                "",
                end_date_text,
                utils::fst(AddDepositMessage::Readonly),
            )
            .width(Length::Units(100)),
        )
}

fn view_currency_duration_percentage_minimual_sum<'a>(
    add_deposit: &state::AddDeposit,
    add_deposit_state: &state::AddDepositState,

    currency: &'a mut pick_list::State<String>,
    duration: &'a mut pick_list::State<String>,
    percentage: &'a mut text_input::State,
    min_amount: &'a mut text_input::State,
) -> Row<'a, AddDepositMessage> {
    let canvass = common::canvass("");
    let (start_date_text, end_date_text) = {
        match add_deposit.deposit_program(add_deposit_state) {
            None => ("", ""),
            Some(ref deposit_program) => (
                &deposit_program.start_date[..],
                &deposit_program.end_date[..],
            ),
        }
    };

    canvass
        .push(
            TextInput::new(
                start_date,
                "",
                start_date_text,
                utils::fst(AddDepositMessage::Readonly),
            )
            .width(Length::Units(100)),
        )
        .push(
            TextInput::new(
                end_date,
                "",
                end_date_text,
                utils::fst(AddDepositMessage::Readonly),
            )
            .width(Length::Units(100)),
        )
}
