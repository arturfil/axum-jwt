#[derive(Debug, Clone)]
pub struct DBConfig {
    pub database_url: String,
    pub redis_url: String,
    pub client_origin: String,
}
