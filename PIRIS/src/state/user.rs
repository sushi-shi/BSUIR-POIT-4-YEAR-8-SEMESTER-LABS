use crate::forms;

use chrono::naive::NaiveDate;
use iced_aw::date_picker::Date;
use postgres::{types::ToSql, Client, Row};

pub struct User {
    pub surname: String,
    pub name: String,
    pub father_name: String,
    pub birthday: Date,
    pub gender: i32,
    pub passport_series: String,
    pub passport_number: String,
    pub passport_issued_by: String,
    pub passport_issued_on: Date,
    pub passport_idx_number: String,
    pub birth_place: String,
    pub current_town: i32,
    pub current_address: String,
    pub mobile_phone: Option<String>,
    pub home_phone: Option<String>,
    pub email: Option<String>,
    pub marital_status: i32,
    pub citizenship: i32,
    pub disability: i32,
    pub is_pensioner: bool,
    pub monthly_income: Option<String>,

    pub registration_town: i32,
}

impl User {
    #[rustfmt::skip]
    pub fn from_form(user: &forms::user::User) -> User {
        let not_empty = |s: String| if s.is_empty() { None } else { Some(s) };
        let pick_no = |(val, vals): (Option<String>, Vec<String>)| vals.into_iter().position(|v| Some(v) == val).unwrap() as i32;

        let fields = &user.fields;

        let surname             = fields[0].widget.text();
        let name                = fields[1].widget.text();
        let father_name         = fields[2].widget.text();
        let birthday            = fields[3].widget.date().unwrap();
        let gender              = fields[4].widget.radio();
        let passport_series     = fields[5].widget.text();
        let passport_number     = fields[6].widget.text();
        let passport_issued_by  = fields[7].widget.text();
        let passport_issued_on  = fields[8].widget.date().unwrap();
        let passport_idx_number = fields[9].widget.text();
        let birth_place         = fields[10].widget.text();
        let current_town        = fields[11].widget.pick();
        let current_address     = fields[12].widget.text();
        let home_phone          = fields[14].widget.text();
        let mobile_phone        = fields[13].widget.text();
        let email               = fields[15].widget.text();
        let registration_town   = fields[16].widget.pick();
        let marital_status      = fields[17].widget.pick();
        let citizenship         = fields[18].widget.pick();
        let disability          = fields[19].widget.pick();
        let is_pensioner        = fields[20].widget.checkbox();
        let monthly_income      = fields[21].widget.text();

        let home_phone          = not_empty(home_phone);
        let mobile_phone        = not_empty(mobile_phone);
        let email               = not_empty(email);
        let monthly_income      = not_empty(monthly_income);

        let current_town        = pick_no(current_town);
        let registration_town   = pick_no(registration_town);
        let marital_status      = pick_no(marital_status);
        let citizenship         = pick_no(citizenship);
        let disability          = pick_no(disability);

        let gender              = gender.0 as i32;

        User {
            surname,
            name,
            father_name,
            birthday,
            gender,
            passport_series,
            passport_number,
            passport_issued_by,
            passport_issued_on,
            passport_idx_number,
            birth_place,
            current_town,
            current_address,
            mobile_phone,
            home_phone,
            email,
            marital_status,
            citizenship,
            disability,
            is_pensioner,
            monthly_income,
            registration_town,
        }
    }

    #[rustfmt::skip]
    pub fn from_row(row: Row) -> User {
        User {
            surname:             row.get(0),
            name:                row.get(1),
            father_name:         row.get(2),
            birthday:            row.get::<_, NaiveDate>(3).into(),
            gender:              row.get(4),
            passport_series:     row.get(5),
            passport_number:     row.get(6),
            passport_issued_by:  row.get(7),
            passport_issued_on:  row.get::<_, NaiveDate>(8).into(),
            passport_idx_number: row.get(9),
            birth_place:         row.get(10),
            current_town:        row.get(11),
            current_address:     row.get(12),
            mobile_phone:        row.get(13),
            home_phone:          row.get(14),
            email:               row.get(15),
            marital_status:      row.get(16),
            citizenship:         row.get(17),
            disability:          row.get(18),
            is_pensioner:        row.get(19),
            monthly_income:      row.get(20),

            registration_town: 0,
        }
    }

    pub fn insert(self, client: &mut Client) -> Result<u64, postgres::Error> {
        let User {
            surname,
            name,
            father_name,
            birthday,
            gender,
            passport_series,
            passport_number,
            passport_issued_by,
            passport_issued_on,
            passport_idx_number,
            birth_place,
            current_town,
            current_address,
            mobile_phone,
            home_phone,
            email,
            marital_status,
            citizenship,
            disability,
            is_pensioner,
            monthly_income,
            ..
        } = &self;

        let birthday: NaiveDate = (*birthday).into();
        let passport_issued_on: NaiveDate = (*passport_issued_on).into();

        let values: &[&(dyn ToSql + Sync)] = &[
            surname,
            name,
            father_name,
            &birthday,
            &gender,
            passport_series,
            passport_number,
            passport_issued_by,
            &passport_issued_on,
            passport_idx_number,
            birth_place,
            &current_town,
            current_address,
            mobile_phone,
            home_phone,
            email,
            marital_status,
            citizenship,
            disability,
            is_pensioner,
            monthly_income,
        ];

        let res = client.execute(include_str!("../../sql/insert_new_user.sql"), values);
        res
    }

    pub fn delete(id: i32, client: &mut Client) -> Result<u64, postgres::Error> {
        let values: &[&(dyn ToSql + Sync)] = &[&id];
        let res = client.execute(include_str!("../../sql/delete_user.sql"), values);
        res
    }

    pub fn update(self, id: i32, client: &mut Client) -> Result<u64, postgres::Error> {
        let User {
            surname,
            name,
            father_name,
            birthday,
            gender,
            passport_series,
            passport_number,
            passport_issued_by,
            passport_issued_on,
            passport_idx_number,
            birth_place,
            current_town,
            current_address,
            mobile_phone,
            home_phone,
            email,
            marital_status,
            citizenship,
            disability,
            is_pensioner,
            monthly_income,
            ..
        } = &self;

        let birthday: NaiveDate = (*birthday).into();
        let passport_issued_on: NaiveDate = (*passport_issued_on).into();

        let values: &[&(dyn ToSql + Sync)] = &[
            surname,
            name,
            father_name,
            &birthday,
            &gender,
            passport_series,
            passport_number,
            passport_issued_by,
            &passport_issued_on,
            passport_idx_number,
            birth_place,
            &current_town,
            current_address,
            mobile_phone,
            home_phone,
            email,
            marital_status,
            citizenship,
            disability,
            is_pensioner,
            monthly_income,
            &id,
        ];

        let res = client.execute(include_str!("../../sql/update_user.sql"), values);
        res
    }
}
