use crate::graphql::types::game_data::GameDataType;
use async_graphql::*;
use entity::game_data;
use entity::game_data::Entity as GameData;
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[derive(Default)]
pub struct GameDataQuery;

#[Object]
impl GameDataQuery {
    pub async fn game_data<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        game_id: i32,
        key: Option<String>,
    ) -> FieldResult<Vec<GameDataType>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let mut query = GameData::find().filter(game_data::Column::GameId.eq(game_id));
        if key.is_some() {
            query = query.filter(game_data::Column::Key.eq(key.unwrap()));
        }
        query = query.order_by_asc(game_data::Column::Key);
        let data: Vec<game_data::Model> = query.all(db).await.expect("Unable to fetch game data");
        Ok(data.into_iter().map(|d| d.into()).collect())
    }

    pub async fn test_game_data(&self) -> FieldResult<GameDataType> {
        Ok(GameDataType::sample())
    }
}
