use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationAccount {
    pub email: String,
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredUser {
    pub email: String,
    pub name: String,
}
