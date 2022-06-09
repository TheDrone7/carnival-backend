use crate::graphql::types::game_api_info::GameApiInfoType;
use async_graphql::*;
use entity::{game_api_info, game_api_info::Entity as GameApiInfo};
use sea_orm::{entity::*, DatabaseConnection};

#[derive(Default)]
pub struct GameApiInfoQuery;

#[Object]
impl GameApiInfoQuery {
    pub async fn game_api_info<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i32,
    ) -> FieldResult<GameApiInfoType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let result: Option<game_api_info::Model> = GameApiInfo::find_by_id(id).one(db).await?;
        if result.is_some() {
            let result_game_api_info = result.unwrap().into();
            Ok(result_game_api_info)
        } else {
            Err(FieldError::new("Invalid ID, game api info not found."))
        }
    }

    pub async fn test_game_api(&self) -> FieldResult<GameApiInfoType> {
        Ok(GameApiInfoType::sample())
    }
}
