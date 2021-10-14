use anyhow::Error;
use sea_orm::entity::prelude::*;
use sea_orm::entity::{Set, Unset};

mod database;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let database = database::get_connection().await?;

    let new_repository = database::entities::repository::ActiveModel {
        id: Set("some_id".to_string()),
        name: Set("some_name".to_string()),
        owner: Set("some_owner".to_string()),
        description: Unset(None)
    };
    let res = new_repository.insert(&database).await?;
    println!("{:#?}", res);
    Ok(())
}
