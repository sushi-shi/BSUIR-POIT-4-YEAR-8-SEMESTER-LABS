pub type Validation = &'static dyn Fn(&str) -> bool;

pub fn is_not_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

pub fn is_a_single_word(value: &str) -> bool {
    let is_a_single_word = value.trim().chars().all(char::is_alphabetic);
    is_a_single_word && value.len() > 1
}

// This method can be optimized but meh
pub fn is_passport_series(value: &str) -> bool {
    ["AB", "BM", "HB", "KH", "MP", "MC", "KB", "PP", "SP", "DP"].contains(&value)
}

pub fn is_passport_number(value: &str) -> bool {
    let is_numeric = value.trim().chars().all(|c| c.is_ascii_digit());
    let is_seven_letters = value.trim().chars().count() == 7;
    is_numeric && is_seven_letters
}

pub fn is_multiple_words(value: &str) -> bool {
    let is_multiple_words = value.trim().split_whitespace().count() > 1;
    is_multiple_words
}

pub fn is_ident_number(value: &str) -> bool {
    let value: Vec<char> = value.trim().chars().collect();
    let correct_length = value.len() == 14;
    if !correct_length {
        return false;
    }

    let gender = value[0];
    let date = &value[1..7];
    let region = value[7];
    let no = &value[8..11];
    let citizenship: String = value[11..13].iter().collect();
    let control_no = value[13];

    let citizenship: &str = &citizenship;

    let is_gender = gender.is_numeric();
    let is_numeric = date.iter().cloned().all(char::is_numeric);
    let is_region = "ABCHKEM".contains(region);
    let is_no = no.iter().cloned().all(char::is_numeric);
    let is_citizenship = ["BA", "PB", "BI"].contains(&citizenship);
    let is_control_no = control_no.is_numeric();

    is_gender && is_numeric && is_region && is_no && is_citizenship && is_control_no
}

pub fn is_phone_number(value: &str) -> bool {
    const COUNTRY_CODE: &str = "+375";
    const OPERATOR_CODES: [&str; 4] = ["17", "25", "29", "33"];
    const NUMBER_LEN: usize = 7;
    const WHOLE_NUMBER_LEN: usize = COUNTRY_CODE.len() + OPERATOR_CODES[0].len() + NUMBER_LEN;

    let value = value.trim();
    if value.len() != WHOLE_NUMBER_LEN {
        return false;
    }

    let has_country_code = &value[0..4] == COUNTRY_CODE;

    let operator_code = &&value[4..6];
    let has_operator_code = OPERATOR_CODES.contains(operator_code);

    let is_numeric = value[6..WHOLE_NUMBER_LEN]
        .chars()
        .all(|c| c.is_ascii_digit());

    has_country_code && has_operator_code && is_numeric
}

pub fn is_email(value: &str) -> bool {
    let parts: Vec<&str> = value.trim().split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    let is_alphanumeric_ascii = |c: char| c.is_alphanumeric() && c.is_ascii();
    let is_correct_local =
        local.chars().next().unwrap().is_alphabetic() && local.chars().all(is_alphanumeric_ascii);

    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() != 2 {
        return false;
    }
    let (addr, domain) = (parts[0], parts[1]);
    if addr.is_empty() || domain.is_empty() {
        return false;
    }
    let is_correct_domain =
        addr.chars().all(is_alphanumeric_ascii) && domain.chars().all(is_alphanumeric_ascii);

    is_correct_domain && is_correct_local
}

pub fn is_money(value: &str) -> bool {
    let is_numeric = value.trim().chars().all(|c| c.is_ascii_digit());
    is_numeric
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_ident_number_test() {
        assert!(is_ident_number("4110873B058PB8"));
        assert!(is_ident_number("4110873A058PB8"));
        assert!(is_ident_number("4110873B058BI4"));
        assert!(is_ident_number("4110873B058BI4"));
        assert!(is_ident_number("4110873B058PB8"));
        assert!(is_ident_number("4100873A058BI4"));
        assert!(is_ident_number("4090873B058BI0"));
        assert!(is_ident_number("5050873A058BI7"));
        assert!(is_ident_number("5020873B058BI6"));
        assert!(is_ident_number("4100873B058BI3"));
        assert!(is_ident_number("4110873A058BI1"));
        assert!(is_ident_number("4110872H058BI2"));
        assert!(is_ident_number("4110873B058PB5"));
    }
}
