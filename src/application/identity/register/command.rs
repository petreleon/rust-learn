use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterCommand {
    pub email: String,
    pub password: String,
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
}
