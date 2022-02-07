use crate::{
    forms::{
        field::{Field, FieldMessage},
        widget::Widget,
    },
    state::{self, common::*, validation::*},
};
use iced::{scrollable, Element, Row, Scrollable};

pub struct User {
    scrollable: scrollable::State,
    pub fields: Vec<Field>,
}

pub type UserMessage = FieldMessage;

impl User {
    // some fields are static, even though they might become dynamic later in the future
    pub fn new() -> User {
        #[rustfmt::skip]
        let fields = vec![
            ("Фамилия", true, Widget::new_text(empty(), &is_a_single_word)),
            ("Имя", true, Widget::new_text(empty(), &is_a_single_word)),
            ("Отчество", true, Widget::new_text(empty(), &is_a_single_word)),
            ("Дата рождения", true, Widget::new_date(None)),
            ("Пол", true, Widget::new_radio(0, Type::Gender.all())),
            ("Серия паспорта", true, Widget::new_text(empty(), &is_passport_series)),
            ("№ паспорта", true, Widget::new_text(empty(), &is_passport_number)),
            ("Кем выдан", true, Widget::new_text(empty(), &is_multiple_words)),
            ("Дата выдачи", true, Widget::new_date(None)),
            ("Идент. номер", true, Widget::new_text(empty(), &is_ident_number)),
            ("Место рождения", true, Widget::new_text(empty(), &is_not_empty)),
            ("Город факт. проживания", true, Widget::new_pick(None, Type::Town.all())),
            ("Адрес факт. проживания", true, Widget::new_text(empty(), &is_not_empty)),
            ("Телефон дом.", false, Widget::new_text(empty(), &is_phone_number)),
            ("Телефон моб.", false, Widget::new_text(empty(), &is_phone_number)),
            ("E-mail", false, Widget::new_text(empty(), &is_email)),
            ("Город прописки", true, Widget::new_pick(None, Type::Town.all())),
            ("Семейное положение", true, Widget::new_pick(None, Type::MaritalStatus.all())),
            ("Гражданство", true, Widget::new_pick(None, Type::Citizenship.all())),
            ("Инвалидность", true, Widget::new_pick(None, Type::Disability.all())),
            ("Пенсионер", true, Widget::new_checkbox("Пенсионер".to_string(), false)),
            ("Ежемесячный доход", false, Widget::new_text(empty(), &is_money)),
        ]
        .into_iter()
        .enumerate()
        .map(|(id, (label, is_required, widget))| Field::new(id, label, is_required, widget))
        .collect();

        User {
            scrollable: scrollable::State::new(),
            fields,
        }
    }

    pub fn from_state(user: state::user::User) -> User {
        #[rustfmt::skip]
        let fields = vec![
            ("Фамилия", true, Widget::new_text(user.surname, &is_a_single_word)),
            ("Имя", true, Widget::new_text(user.name, &is_a_single_word)),
            ("Отчество", true, Widget::new_text(user.father_name, &is_a_single_word)),
            ("Дата рождения", true, Widget::new_date(Some(user.birthday))),
            ("Пол", true, Widget::new_radio(user.gender as usize, Type::Gender.all())),
            ("Серия паспорта", true, Widget::new_text(user.passport_series, &is_passport_series)),
            ("№ паспорта", true, Widget::new_text(user.passport_number, &is_passport_number)),
            ("Кем выдан", true, Widget::new_text(user.passport_issued_by, &is_multiple_words)),
            ("Дата выдачи", true, Widget::new_date(Some(user.passport_issued_on))),
            ("Идент. номер", true, Widget::new_text(user.passport_idx_number, &is_ident_number)),
            ("Место рождения", true, Widget::new_text(user.birth_place, &is_not_empty)),
            ("Город факт. проживания", true, Widget::new_pick(Some(Type::Town.no(user.current_town)), Type::Town.all())),
            ("Адрес факт. проживания", true, Widget::new_text(user.current_address, &is_not_empty)),
            ("Телефон дом.", false, Widget::new_text(user.home_phone.unwrap_or_default(), &is_phone_number)),
            ("Телефон моб.", false, Widget::new_text(user.mobile_phone.unwrap_or_default(), &is_phone_number)),
            ("E-mail", false, Widget::new_text(user.email.unwrap_or_default(), &is_email)),
            ("Город прописки", true, Widget::new_pick(Some(Type::Town.no(user.registration_town)), Type::Town.all())),
            ("Семейное положение", true, Widget::new_pick(Some(Type::MaritalStatus.no(user.marital_status)), Type::MaritalStatus.all())),
            ("Гражданство", true, Widget::new_pick(Some(Type::Citizenship.no(user.citizenship)), Type::Citizenship.all())),
            ("Инвалидность", true, Widget::new_pick(Some(Type::Disability.no(user.disability)), Type::Disability.all())),
            ("Пенсионер", true, Widget::new_checkbox("Пенсионер".to_string(), user.is_pensioner)),
            ("Ежемесячный доход", false, Widget::new_text(user.monthly_income.unwrap_or_default(), &is_money)),
        ]
        .into_iter()
        .enumerate()
        .map(|(id, (label, is_required, widget))| Field::new(id, label, is_required, widget))
        .collect();

        User {
            scrollable: scrollable::State::new(),
            fields,
        }
    }

    pub fn verify_static(&mut self) -> Vec<usize> {
        for field in &mut self.fields {
            field.has_failed = false;
        }
        let failure_ids: Vec<usize> = self
            .fields
            .iter()
            .map(Field::has_failed)
            .filter(|(_, x)| *x)
            .map(|(x, _)| x)
            .collect();
        failure_ids
    }

    pub fn verify_dynamic(err: &postgres::Error) -> Vec<usize> {
        let mut vec = Vec::new();
        if let Some(db_error) = err.as_db_error() {
            if let Some(constraint) = db_error.constraint() {
                if constraint.contains("ak_passportnumberunique") {
                    vec.push(6)
                } else {
                    vec.push(9)
                }
            }
        }
        vec
    }

    pub fn view(&mut self) -> Element<UserMessage> {
        let mut column = Scrollable::new(&mut self.scrollable);

        let mut current_row_no = 0;
        let mut current_element_no = 0;
        let mut current_row = Row::new();
        let elements_in_a_row = [5, 5, 3, 4, 5, i32::MAX];

        for field in &mut self.fields {
            if current_element_no == elements_in_a_row[current_row_no] {
                column = column.push(current_row);
                current_row = Row::new();
                current_row_no += 1;
                current_element_no = 0;
            }
            current_row = current_row.push(field.view());
            current_element_no += 1;
        }
        column = column.push(current_row);

        column.into()
    }

    pub fn update(&mut self, message: UserMessage) {
        self.fields[message.id].update(message)
    }
}

fn empty() -> String {
    "".to_string()
}
