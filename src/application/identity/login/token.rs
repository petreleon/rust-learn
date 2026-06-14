use super::LoginError;

pub trait LoginTokenIssuer {
    fn issue_token(&self, user_id: i32) -> Result<String, LoginError>;
}
