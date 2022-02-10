pub mod add_deposit;
use self::add_deposit::{AddDeposit, AddDepositMessage};
use crate::state;

use crate::forms::common::{button, column, row};
use iced::{button, Element, Text};
use postgres::Client;

pub enum DepositsMain {
    DepositsMain {
        add_deposit_button: button::State,
        list_deposts_button: button::State,
        list_bank_accounts_button: button::State,

        back_button: button::State,
    },
    AddDeposit(AddDeposit),
}

impl Default for DepositsMain {
    fn default() -> Self {
        Self::DepositsMain {
            add_deposit_button: button::State::default(),
            list_deposts_button: button::State::default(),
            list_bank_accounts_button: button::State::default(),

            back_button: button::State::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DepositsMainMessage {
    // DepositsMain messages
    AddDepositClicked,
    ListDepositsClicked,
    ListBankAccountsClicked,
    BackClicked,
    // Routing messages
    AddDeposit(AddDepositMessage),
}

impl DepositsMain {
    pub fn view(&mut self, client: &mut Client) -> Element<DepositsMainMessage> {
        match self {
            Self::DepositsMain {
                ref mut add_deposit_button,
                ref mut list_deposts_button,
                ref mut list_bank_accounts_button,
                ref mut back_button,
            } => {
                let mut col = column();
                let rw = row()
                    .push(Text::new("Работа с депозитами").size(40))
                    .push(button(
                        "Назад",
                        DepositsMainMessage::BackClicked,
                        back_button,
                    ));
                col = col.push(rw);

                let rw = row()
                    .push(button(
                        "Оформление депозита",
                        DepositsMainMessage::AddDepositClicked,
                        add_deposit_button,
                    ))
                    .push(button(
                        "Депозиты",
                        DepositsMainMessage::ListDepositsClicked,
                        list_deposts_button,
                    ));
                col = col.push(rw);

                let rw = row().push(button(
                    "Список счетов",
                    DepositsMainMessage::ListBankAccountsClicked,
                    list_bank_accounts_button,
                ));
                col.push(rw).into()
            }
            Self::AddDeposit(add_deposit) => add_deposit
                .view(client)
                .map(DepositsMainMessage::AddDeposit),
        }
    }

    pub fn update(&mut self, message: DepositsMainMessage, client: &mut Client) -> bool {
        let mut is_finished = false;
        match message {
            // DepositsMain messages
            DepositsMainMessage::BackClicked => is_finished = true,
            DepositsMainMessage::AddDepositClicked => {
                let add_deposit = AddDeposit::new(client);
                *self = Self::AddDeposit(add_deposit)
            }
            DepositsMainMessage::ListDepositsClicked => (),
            DepositsMainMessage::ListBankAccountsClicked => (),

            // Routing messages
            DepositsMainMessage::AddDeposit(add_deposit_message) => match self {
                Self::AddDeposit(add_deposit) => {
                    is_finished = add_deposit.update(add_deposit_message, client)
                }
                _ => unreachable!(),
            },
        }
        is_finished
    }
}
