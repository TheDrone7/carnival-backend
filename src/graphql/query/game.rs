use crate::graphql::types::game::GameType;
use async_graphql::*;
use entity::game;
use entity::game::Entity as Game;
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[derive(Default)]
pub struct GameQuery;

#[Object]
impl GameQuery {
    pub async fn game<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> FieldResult<GameType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let games: Vec<game::Model> = Game::find()
            .filter(game::Column::Id.eq(id))
            .order_by_asc(game::Column::Id)
            .all(db)
            .await
            .expect("Unable to fetch games");
        if games.len() > 0 {
            let game = games[0].clone().into();
            Ok(game)
        } else {
            Err(FieldError::new("Game not found"))
        }
    }

    pub async fn test_game(&self) -> FieldResult<GameType> {
        Ok(GameType::sample())
    }
}
