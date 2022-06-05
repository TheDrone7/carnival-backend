use crate::graphql::types::game::{new_game, GameType};
use async_graphql::*;
use sea_orm::{entity::*, DatabaseConnection};

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
        let game = new_game(input).insert(db).await?;
        Ok(game.into())
    }
}
