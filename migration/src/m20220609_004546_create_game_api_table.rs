use sea_orm_migration::prelude::*;

use crate::m20220605_165732_create_game_table::Game;

pub struct Migration;

#[derive(Iden)]
enum GameApiInfo {
    Table,
    GameId,
    GameSecret,
    GameToken,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220609_004546_create_game_api_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GameApiInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GameApiInfo::GameId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GameApiInfo::GameSecret).string().not_null())
                    .col(ColumnDef::new(GameApiInfo::GameToken).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(GameApiInfo::Table, GameApiInfo::GameId)
                    .to(Game::Table, Game::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(ForeignKey::drop().table(GameApiInfo::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GameApiInfo::Table).to_owned())
            .await
    }
}
