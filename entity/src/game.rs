//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "game")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub repl_url: String,
    pub user_id: i32,
    pub icon_url: String,
    pub cover_url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(has_many = "super::game_data::Entity")]
    GameData,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::game_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GameData.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
