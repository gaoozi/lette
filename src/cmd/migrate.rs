use crate::conf::Conf;
use anyhow::Ok;
use clap::Args;
use sqlx::mysql::MySqlPoolOptions;

#[derive(Debug, Args)]
pub struct Cmd {}

pub fn handle(_cmd: &Cmd, conf: &Conf) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let pool = MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&conf.db.url())
                .await
                .expect("Database connection failed");

            sqlx::migrate!()
                .run(&pool)
                .await
                .expect("Database migrate failed");
        });

    Ok(())
}
