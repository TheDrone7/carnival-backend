use crate::graphql::types::game_api_info::GameApiInfoType;
use async_graphql::*;
use entity::{
    game, game::Entity as Game, game_api_info, game_api_info::Entity as GameApiInfo,
    user::Model as UserModel,
};
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
        let current_user = ctx.data_unchecked::<Option<UserModel>>();
        if current_user.is_none() {
            return FieldResult::Err(FieldError::new("You are not logged in."));
        }
        let current_user = current_user.clone().unwrap();
        let req_game: Option<game::Model> = Game::find_by_id(id).one(db).await?;
        if req_game.is_none() {
            return FieldResult::Err(FieldError::new("Game not found."));
        }
        let req_game = req_game.unwrap();
        if req_game.user_id != current_user.id {
            return Err(FieldError::new("You are not authorized."));
        }
        let result: Option<game_api_info::Model> = GameApiInfo::find_by_id(id).one(db).await?;
        if result.is_none() {
            return FieldResult::Err(FieldError::new("Game not found."));
        }
        Ok(result.unwrap().into())
    }

    pub async fn sample_game_api(&self) -> FieldResult<GameApiInfoType> {
        Ok(GameApiInfoType::sample())
    }
}
