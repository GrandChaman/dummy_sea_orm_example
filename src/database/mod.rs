pub mod entities;

use anyhow::Error;
use sea_orm::{Database, DatabaseConnection};

pub async fn get_connection() -> Result< DatabaseConnection, Error>
{
    // TODO Config
    Ok(Database::connect("sqlite://data.db").await?)
}
