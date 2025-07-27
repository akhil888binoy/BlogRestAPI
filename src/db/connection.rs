use sea_orm::{Database, DatabaseConnection, DbErr};
use dotenv::dotenv;
use std::env;

use crate::models::db_models::DbConnection;


pub async fn connect() -> Result<DbConnection, DbErr> {
    dotenv().ok();
    let dbstring = env::var("DATABASE_URL").expect("No DB Url found");
    let db = Database::connect(dbstring).await?;
    Ok(DbConnection{db})
}

