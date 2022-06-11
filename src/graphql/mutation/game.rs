use crate::graphql::types::game::{new_game, GameConfig, GameType};
use async_graphql::*;
use entity::{
    game::Entity as Game, game_data, game_data::Entity as GameData, user::Model as UserModel,
};
use sea_orm::{entity::*, DatabaseConnection, QueryFilter};

#[derive(Default)]
pub struct GameMutation;

#[Object]
impl GameMutation {
    pub async fn create_game<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: GameType,
    ) -> FieldResult<GameType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let some_user = ctx.data_unchecked::<Option<UserModel>>();
        if some_user.is_none() {
            return Err(FieldError::new("You are not logged in."));
        }
        if input.user_id != some_user.clone().unwrap().id {
            return Err(FieldError::new("Bad request. Invalid input."));
        }
        let game = new_game(input).insert(db).await?;
        Ok(game.into())
    }

    pub async fn update_game<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i32,
        input: GameConfig,
    ) -> FieldResult<GameType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let some_user = ctx.data_unchecked::<Option<UserModel>>();
        if some_user.is_none() {
            return Err(FieldError::new("You are not logged in."));
        }
        let some_user = some_user.clone().unwrap();
        let user_id = some_user.id;
        let some_game = Game::find_by_id(id).one(db).await?;
        if some_game.is_none() {
            return Err(FieldError::new("Game not found."));
        }
        let some_game = some_game.unwrap();
        if some_game.user_id != user_id {
            return Err(FieldError::new("You are not authorized."));
        }
        let mut some_game = some_game.into_active_model();
        if input.title.is_some() {
            some_game.title = Set(input.title.unwrap());
        }
        if input.description.is_some() {
            some_game.description = Set(input.description.unwrap());
        }
        if input.repl_url.is_some() {
            some_game.repl_url = Set(input.repl_url.unwrap());
        }
        if input.icon_url.is_some() {
            some_game.icon_url = Set(input.icon_url.unwrap());
        }
        if input.cover_url.is_some() {
            some_game.cover_url = Set(input.cover_url.unwrap());
        }
        let some_game = some_game.update(db).await?;
        Ok(some_game.into())
    }

    pub async fn delete_game<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> FieldResult<GameType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let some_user = ctx.data_unchecked::<Option<UserModel>>();
        if some_user.is_none() {
            return Err(FieldError::new("You are not logged in."));
        }
        let user_id = some_user.clone().unwrap().id;
        let some_game = Game::find_by_id(id).one(db).await?;
        if some_game.is_none() {
            return Err(FieldError::new("Game not found."));
        }
        let some_game = some_game.unwrap();
        if some_game.user_id != user_id {
            return Err(FieldError::new("You are not authorized."));
        }
        GameData::delete_many()
            .filter(game_data::Column::GameId.eq(id))
            .exec(db)
            .await?;
        let res = some_game.clone().delete(db).await?;
        if res.rows_affected < 1 {
            return Err(FieldError::new("Game not found."));
        }
        Ok(some_game.into())
    }
}
