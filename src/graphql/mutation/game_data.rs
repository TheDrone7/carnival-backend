use crate::graphql::types::game_data::{new_game_data, GameDataType};
use async_graphql::*;
use entity::game_data;
use entity::game_data::Entity as GameData;
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[derive(Default)]
pub struct GameDataMutation;

#[Object]
impl GameDataMutation {
    pub async fn set_game_data<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        game_id: i32,
        key: String,
        value: String,
    ) -> FieldResult<GameDataType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let data: Vec<game_data::Model> = GameData::find()
            .filter(game_data::Column::GameId.eq(game_id))
            .filter(game_data::Column::Key.eq(key.clone()))
            .order_by_asc(game_data::Column::Id)
            .all(db)
            .await
            .expect("Unable to fetch game data");
        let mut new_data: game_data::ActiveModel;
        let result_data: game_data::Model;
        if data.is_empty() {
            result_data = new_game_data(GameDataType {
                game_id,
                key: key.clone(),
                value: Some(value),
            })
            .insert(db)
            .await?;
        } else {
            new_data = data[0].clone().into();
            new_data.value = Set(Some(value));
            result_data = new_data.update(db).await?;
        }
        Ok(result_data.into())
    }
}
