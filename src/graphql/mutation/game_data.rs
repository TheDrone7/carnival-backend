use crate::graphql::types::game_data::{new_game_data, GameDataType};
use async_graphql::*;
use entity::game;
use entity::game_data;
use entity::game_data::Entity as GameData;
use sea_orm::{entity::*, query::*, DatabaseConnection, DeleteResult};

#[derive(Default)]
pub struct GameDataMutation;

#[Object]
impl GameDataMutation {
    pub async fn set_game_data<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        key: String,
        value: String,
    ) -> FieldResult<GameDataType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let some_game = ctx.data_unchecked::<Option<game::Model>>();
        if some_game.is_none() {
            return Err(FieldError::new("Invalid API Key"));
        }
        let game_id = some_game.clone().unwrap().id;
        let data: Vec<game_data::Model> = GameData::find()
            .filter(game_data::Column::GameId.eq(game_id))
            .filter(game_data::Column::Key.eq(key.clone()))
            .order_by_asc(game_data::Column::Id)
            .all(db)
            .await
            .expect("Unable to fetch game data");

        let mut new_data: game_data::ActiveModel;
        let result_data: game_data::Model = if data.is_empty() {
            new_game_data(GameDataType {
                game_id,
                key: key.clone(),
                value: Some(value),
            })
            .insert(db)
            .await?
        } else {
            new_data = data[0].clone().into();
            new_data.value = Set(Some(value));
            new_data.update(db).await?
        };
        Ok(result_data.into())
    }

    pub async fn delete_game_data<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        key: Option<String>,
    ) -> FieldResult<Vec<GameDataType>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let some_game = ctx.data_unchecked::<Option<game::Model>>();
        if some_game.is_none() {
            return Err(FieldError::new("Invalid API Key"));
        }
        let game_id = some_game.clone().unwrap().id;
        let mut query = GameData::find().filter(game_data::Column::GameId.eq(game_id));
        let some_key = key.clone();
        if let Some(some_key) = some_key {
            query = query.filter(game_data::Column::Key.eq(some_key));
        }
        let data: Vec<game_data::Model> = query
            .order_by_asc(game_data::Column::Id)
            .all(db)
            .await
            .expect("Unable to fetch game data");

        if data.is_empty() {
            return Err(FieldError::new("No relevant data found."));
        } else {
            let mut query = GameData::delete_many().filter(game_data::Column::GameId.eq(game_id));
            let some_key = key.clone();
            if let Some(some_key) = some_key {
                query = query.filter(game_data::Column::Key.eq(some_key));
            }
            let res: DeleteResult = query.exec(db).await?;
            if res.rows_affected != data.len() as u64 {
                return Err(FieldError::new("Unable to delete game data."));
            }
        }
        Ok(data.into_iter().map(|d| d.into()).collect())
    }
}
