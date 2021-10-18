mod api_key;
mod openid;

pub use self::api_key::*;
pub use self::openid::*;

use crate::error::ClientError;
use async_trait::async_trait;

#[async_trait]
pub trait TokenProvider {
    type Error: std::error::Error + Send + Sync;

    async fn provide_access_token(
        &self,
    ) -> Result<Option<String>, crate::error::ClientError<Self::Error>>;
}

pub struct NoTokenProvider;

#[async_trait]
impl TokenProvider for NoTokenProvider {
    type Error = reqwest::Error;

    async fn provide_access_token(&self) -> Result<Option<String>, ClientError<Self::Error>> {
        Ok(None)
    }
}
