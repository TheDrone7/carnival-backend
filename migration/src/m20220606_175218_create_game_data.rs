use crate::m20220605_165732_create_game_table::Game;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220606_175218_create_game_data"
    }
}

#[derive(Iden)]
pub enum GameData {
    Table,
    GameId,
    Key,
    Value,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GameData::Table)
                    .col(ColumnDef::new(GameData::GameId).integer().not_null())
                    .col(ColumnDef::new(GameData::Key).string().not_null())
                    .col(ColumnDef::new(GameData::Value).text())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(GameData::Table, GameData::GameId)
                    .to(Game::Table, Game::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(ForeignKey::drop().table(GameData::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GameData::Table).to_owned())
            .await
    }
}
