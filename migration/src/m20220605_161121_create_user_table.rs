use sea_orm_migration::prelude::*;

pub struct Migration;

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    AvatarUrl,
    FullName,
    Bio,
    IsHacker,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220605_161121_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::AvatarUrl).string().not_null())
                    .col(ColumnDef::new(User::FullName).string().not_null())
                    .col(ColumnDef::new(User::Bio).string().not_null())
                    .col(ColumnDef::new(User::IsHacker).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
