use sea_orm_migration::prelude::*;

pub struct Migration;

#[derive(Iden)]
pub enum Game {
    Table,
    Id,
    Title,
    Description,
    ReplUrl,
    UserId,
    IconUrl,
    CoverUrl,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220605_165732_create_game_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::Title).string().not_null())
                    .col(ColumnDef::new(Game::Description).string().not_null())
                    .col(ColumnDef::new(Game::ReplUrl).string().not_null())
                    .col(ColumnDef::new(Game::UserId).integer().not_null())
                    .col(ColumnDef::new(Game::IconUrl).string().not_null())
                    .col(ColumnDef::new(Game::CoverUrl).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}
