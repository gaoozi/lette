use std::env;

use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Conf {
    pub db: Database,
    pub app: Application,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Application {
    pub port: u16,
    pub host: String,
    pub secret: Secret<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub user: String,
    pub passwd: Secret<String>,
    pub port: u16,
    pub host: String,
    pub name: String,
    pub ssl: bool,
}

impl Database {
    pub fn url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user,
            self.passwd.expose_secret(),
            self.host,
            self.port,
            self.name
        )
    }
}

pub fn setup() -> anyhow::Result<Conf> {
    let base_path = env::current_dir().expect("Failed to determine the current dir");
    let conf_path = base_path.join("config");

    let app_env: Env = env::var("APP_ENV")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to parse APP_ENV");

    let conf = config::Config::builder()
        .add_source(config::File::from(conf_path.join("base.toml")))
        .add_source(config::File::from(
            conf_path.join(format!("{}.toml", app_env.as_str())),
        ))
        .add_source(
            config::Environment::with_prefix("LETTE")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    Ok(conf.try_deserialize()?)
}

pub enum Env {
    Dev,
    Prod,
}

impl Env {
    pub fn as_str(&self) -> &'static str {
        match self {
            Env::Dev => "dev",
            Env::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            other => Err(format!("{} is not supported, Use `dev` or `prod`", other)),
        }
    }
}
