use sea_orm::DatabaseConnection;



#[derive(Clone)]
pub struct DbConnection{
    pub db: DatabaseConnection
}