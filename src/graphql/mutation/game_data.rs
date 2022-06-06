use crate::graphql::types::game_data::{new_game_data, GameDataType};
use async_graphql::*;
use sea_orm::{entity::*, DatabaseConnection};

#[derive(Default)]
pub struct GameDataMutation;

#[Object]
impl GameDataMutation {
    pub async fn add_game_data<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: GameDataType,
    ) -> FieldResult<GameDataType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let game_data = new_game_data(input).insert(db).await?;
        Ok(game_data.into())
    }
}
