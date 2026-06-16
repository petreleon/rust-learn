use futures::future::BoxFuture;

use crate::application::wallet::burn_tokens::{
    TokenBurnCommand, TokenBurnError, TokenBurnLeaderboard, TokenBurnLeaderboardQuery,
    TokenBurnSubject, TokenBurnView,
};

pub trait TokenBurnUseCase: Send + Sync {
    fn request_token_burn(
        &self,
        actor_user_id: i32,
        subject: TokenBurnSubject,
        command: TokenBurnCommand,
    ) -> BoxFuture<'_, Result<TokenBurnView, TokenBurnError>>;

    fn list_token_burns(
        &self,
        actor_user_id: i32,
        subject: TokenBurnSubject,
    ) -> BoxFuture<'_, Result<Vec<TokenBurnView>, TokenBurnError>>;

    fn load_token_burn_leaderboard(
        &self,
        actor_user_id: i32,
        query: TokenBurnLeaderboardQuery,
    ) -> BoxFuture<'_, Result<TokenBurnLeaderboard, TokenBurnError>>;
}
