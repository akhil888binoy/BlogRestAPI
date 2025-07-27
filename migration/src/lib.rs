pub use sea_orm_migration::prelude::*;

mod m20250715_121626_create_posts_table;
mod m20250724_115046_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250715_121626_create_posts_table::Migration),
            Box::new(m20250724_115046_create_user_table::Migration),
        ]
    }
}
