use serde::{Deserialize, Serialize};

use crate::application::identity::list_users::ListUsersQuery;
use crate::application::identity::user_profile::UserProfileOutput;

#[derive(Debug, Clone, Deserialize)]
pub struct ListUsersRequest {
    pub search: Option<String>,
}

impl From<ListUsersRequest> for ListUsersQuery {
    fn from(request: ListUsersRequest) -> Self {
        ListUsersQuery::new(request.search)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UsersResponse {
    pub users: Vec<UserProfileResponse>,
}

impl From<Vec<UserProfileOutput>> for UsersResponse {
    fn from(users: Vec<UserProfileOutput>) -> Self {
        Self {
            users: users.into_iter().map(UserProfileResponse::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserProfileResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub date_of_birth: Option<String>,
    pub created_at: String,
    pub kyc_verified: bool,
    pub email_verified: bool,
}

impl From<UserProfileOutput> for UserProfileResponse {
    fn from(user: UserProfileOutput) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            date_of_birth: user.date_of_birth.map(|date| date.to_string()),
            created_at: user.created_at.to_string(),
            kyc_verified: user.kyc_verified,
            email_verified: user.email_verified,
        }
    }
}
