use crate::graphql::types::game_data::{new_game_data, GameDataType};
use async_graphql::*;
use entity::game;
use entity::game::Entity as Game;
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
        game_id: i32,
        key: String,
        value: String,
    ) -> FieldResult<GameDataType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let user_id = ctx.data_opt::<String>();
        if user_id.is_none() {
            return Err(FieldError::new("Please sign-in with replit first."));
        }
        let user_id = user_id.unwrap().parse::<i32>().unwrap();
        let game: Option<game::Model> = Game::find_by_id(game_id).one(db).await?;
        if game.is_none() {
            return Err(FieldError::new("Invalid request, no such game found."));
        } else if game.unwrap().user_id != user_id {
            return Err(FieldError::new(
                "Unauthorized, you are not the owner of this game.",
            ));
        }
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

    pub async fn delete_game_data<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        game_id: i32,
        key: Option<String>,
    ) -> FieldResult<Vec<GameDataType>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let user_id = ctx.data_opt::<String>();
        if user_id.is_none() {
            return Err(FieldError::new("Please sign-in with replit first."));
        }
        let user_id = user_id.unwrap().parse::<i32>().unwrap();
        let game: Option<game::Model> = Game::find_by_id(game_id).one(db).await?;
        if game.is_none() {
            return Err(FieldError::new("Invalid request, no such game found."));
        } else if game.unwrap().user_id != user_id {
            return Err(FieldError::new(
                "Unauthorized, you are not the owner of this game.",
            ));
        }

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
            return Err(FieldError::new("No such data found."));
        } else {
            let mut query = GameData::delete_many().filter(game_data::Column::GameId.eq(game_id));
            let some_key = key.clone();
            if let Some(some_key) = some_key {
                query = query.filter(game_data::Column::Key.eq(some_key));
            }
            let res: DeleteResult = query.exec(db).await.expect("Unable to delete game data");
            if res.rows_affected != data.len() as u64 {
                return Err(FieldError::new("Unable to delete game data."));
            }
        }
        Ok(data.into_iter().map(|d| d.into()).collect())
    }
}
