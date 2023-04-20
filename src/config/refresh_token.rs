#[derive(Debug, Clone)]
pub struct RefreshConfig {
    pub refresh_token_private_key: String,
    pub refresh_token_public_key: String,
    pub refresh_token_expires_in: String,
    pub refresh_token_max_age: i64,
}
