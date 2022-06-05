use crate::m20220605_161121_create_user_table::User;
use crate::m20220605_165732_create_game_table::Game;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220605_171425_add_user_game_relation"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Game::Table, Game::UserId)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(ForeignKey::drop().table(Game::Table).to_owned())
            .await
    }
}
