pub use sea_orm_migration::prelude::*;

mod m20220605_161121_create_user_table;
mod m20220605_165732_create_game_table;
mod m20220605_171425_add_user_game_relation;
mod m20220606_175218_create_game_data;
mod m20220609_004546_create_game_api_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220605_161121_create_user_table::Migration),
            Box::new(m20220605_165732_create_game_table::Migration),
            Box::new(m20220605_171425_add_user_game_relation::Migration),
            Box::new(m20220606_175218_create_game_data::Migration),
            Box::new(m20220609_004546_create_game_api_table::Migration),
        ]
    }
}
