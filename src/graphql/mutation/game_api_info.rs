use crate::graphql::types::game_api_info::{new_game_api_info, GameApiInfoType};
use async_graphql::*;
use entity::{
    game::Entity as Game, game_api_info, game_api_info::Entity as GameApiInfo,
    user::Model as UserModel,
};
use sea_orm::{entity::*, DatabaseConnection};

#[derive(Default)]
pub struct GameApiInfoMutation;

#[Object]
impl GameApiInfoMutation {
    pub async fn generate_api_info<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i32,
    ) -> FieldResult<GameApiInfoType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let user_id = ctx.data_unchecked::<Option<UserModel>>();
        if user_id.is_none() {
            return Err(FieldError::new("You are not logged in."));
        }
        let user_id = user_id.clone().unwrap().id;
        let some_game = Game::find_by_id(id).one(db).await?;
        if some_game.is_none() {
            return Err(FieldError::new("Game not found."));
        }
        let some_game = some_game.unwrap();
        if some_game.user_id != user_id {
            return Err(FieldError::new("You are not authorized."));
        }
        let api_info: Option<game_api_info::Model> = GameApiInfo::find_by_id(id).one(db).await?;
        let new_api_info = new_game_api_info(id);
        let result: game_api_info::Model = if let Some(api_info) = api_info {
            let mut api_info = api_info.into_active_model();
            api_info.game_secret = new_api_info.game_secret;
            api_info.game_token = new_api_info.game_token;
            api_info.update(db).await?
        } else {
            new_api_info.insert(db).await?
        };
        Ok(result.into())
    }
}
