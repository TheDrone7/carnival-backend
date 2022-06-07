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
        let user_id = ctx.data_opt::<String>();
        if user_id.is_none() {
            return Err(FieldError::new("Please sign-in with replit first."));
        }
        let user_id = user_id.unwrap().parse::<i32>().unwrap();
        if input.user_id != user_id {
            return Err(FieldError::new("Invalid request."));
        }
        let game = new_game(input).insert(db).await?;
        Ok(game.into())
    }
}
