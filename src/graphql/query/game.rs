use crate::graphql::types::game::GameType;
use async_graphql::*;
use entity::game;
use entity::game::Entity as Game;
use sea_orm::{entity::*, DatabaseConnection};

#[derive(Default)]
pub struct GameQuery;

#[Object]
impl GameQuery {
    pub async fn game<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> FieldResult<GameType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let result: Option<game::Model> = Game::find_by_id(id).one(db).await?;
        if result.is_some() {
            let result_game = result.unwrap().into();
            Ok(result_game)
        } else {
            Err(FieldError::new("Invalid ID, game not found."))
        }
    }

    pub async fn sample_game(&self) -> FieldResult<GameType> {
        Ok(GameType::sample())
    }
}
