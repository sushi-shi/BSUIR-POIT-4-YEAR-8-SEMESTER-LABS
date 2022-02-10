use crate::state::utils;
use chrono::naive::NaiveDate;

type Duration = i32;
type CurrencyId = i32;
type Percentage = f64;
type Amount = i32;

pub type DepositData = (i32, Duration, Percentage, CurrencyId, Amount);

pub struct DepositProgram {
    pub id: i32,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub is_revokable: bool,
    pub data: Vec<(i32, Duration, Percentage, CurrencyId, Amount)>,
}

pub struct AddDeposit {
    // Vector of users for whom deposits are available
    pub users: Vec<(i32, String)>,
    // Currently running deposit programs
    pub deposit_programs: Vec<DepositProgram>,
}

// Bad things will happen if there are no users or programs, beware!
#[derive(Default)]
pub struct AddDepositState {
    pub client: Option<String>,
    pub deposit_program: Option<String>,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub duration: Option<String>,

    pub deposit_no: String,
}

impl AddDeposit {
    pub fn deposit_program(&self, state: &AddDepositState) -> Option<&DepositProgram> {
        let deposit_program = state.deposit_program.as_ref()?;

        Some(
            self.deposit_programs
                .iter()
                .find(|x| &x.name == deposit_program)
                .unwrap(),
        )
    }

    pub fn build(client: &mut postgres::Client) -> AddDeposit {
        // 1
        let mut users = Vec::new();
        let rows = client
            .query(
                "SELECT id, surname, name, father_name FROM bank.client",
                &[],
            )
            .unwrap();
        for row in rows {
            let id = row.get(0);
            let surname: String = row.get(1);
            let name: String = row.get(2);
            let father_name: String = row.get(3);

            users.push((id, format!("{} {} {}", surname, name, father_name)));
        }

        // 2
        let mut deposit_programs_data: Vec<(i32, DepositData)> = Vec::new();
        let rows = client
            .query(
                include_str!("../../../sql/select_deposit_program_data.sql"),
                &[],
            )
            .unwrap();
        for row in rows {
            let deposit_program_id = row.get(1);
            let deposit_program_data_id = row.get(0);

            deposit_programs_data.push((
                deposit_program_id,
                (
                    deposit_program_data_id,
                    row.get(2),
                    row.get(3),
                    row.get(4),
                    row.get(5),
                ),
            ))
        }
        let deposit_programs_data = utils::group(deposit_programs_data);

        // 3
        let mut deposit_programs = Vec::new();
        let rows = client
            .query(include_str!("../../../sql/select_deposit_program.sql"), &[])
            .unwrap();
        for row in rows {
            let id = row.get(0);
            deposit_programs.push(DepositProgram {
                id,
                name: row.get(1),
                start_date: utils::show(row.get::<_, NaiveDate>(2)),
                end_date: utils::show(row.get::<_, NaiveDate>(3)),
                is_revokable: row.get(4),
                data: utils::find(id, &deposit_programs_data).unwrap(),
            })
        }

        AddDeposit {
            users,
            deposit_programs,
        }
    }
}
