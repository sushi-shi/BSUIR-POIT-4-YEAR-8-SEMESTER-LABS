use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Town,
    Disability,
    MaritalStatus,
    Citizenship,
    Gender,
}

impl Type {
    pub fn no(self, no: i32) -> String {
        self.all()[no as usize].clone()
    }

    pub fn all(self) -> Vec<String> {
        match self {
            Self::Town => Town::all(),
            Self::MaritalStatus => MaritalStatus::all(),
            Self::Disability => Disability::all(),
            Self::Citizenship => Citizenship::all(),
            Self::Gender => Gender::all(),
        }
    }
}

macro_rules! pick_type {
    (pub enum $name:ident { $($fname:ident = $flit:literal ,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
        pub enum $name {
            $($fname),*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        $(Self::$fname => $flit),*
                    }
                )
            }
        }

        impl $name {
            pub fn all() -> Vec<String> {
                Self::iter().map(|x| format!("{}", x)).collect()
            }
        }
    }
}

pick_type!(
    pub enum Town {
        Minsk = "Минск",
        Brest = "Брест",
        Grodno = "Гродно",
        Vitebsk = "Витебск",
        Gomel = "Гомель",
        Mogilev = "Могилев",
    }
);

pick_type!(
    pub enum Disability {
        None = "Отсутсвует",
        First = "Первой степени",
        Second = "Второй степени",
        Third = "Третьей степени",
    }
);

pick_type!(
    pub enum MaritalStatus {
        Single = "Холост",
        Married = "Женат",
        Divorced = "Разведен",
        Widowed = "Овдовевший",
    }
);

pick_type!(
    pub enum Citizenship {
        Belarus = "Беларусь",
        Russia = "Россия",
        Ukraine = "Украина",
    }
);

pick_type!(
    pub enum Gender {
        Female = "Женский",
        Male = "Мужской",
    }
);
