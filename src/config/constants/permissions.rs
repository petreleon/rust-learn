use strum_macros::{Display, EnumString};


#[derive(Display, EnumString, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Permissions{
    CREATE_ORGANIZATION,
    MODIFY_ANY_ORGANIZATION,
    CREATE_COURSE,
    JOIN_COURSE,
    REQUEST_JOIN_COURSE,
    MODIFY_ANY_COURSE,
    MODIFY_ANY_USER,
    MODIFY_ANY_ROLE,
    MODIFY_ANY_PERMISSION,
}
