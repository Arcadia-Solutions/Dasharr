use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct Env {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(from = "API_KEY")]
    pub api_key: String,
}
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("env variable parse error '{0}'")]
    EnvVariableParseError(String),
}
