pub mod api;
pub mod error;
pub mod types;

pub async fn setup_database(pool: &PgPool) -> Result<(), Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}