use async_graphql::*;
use entity::game_data;
use sea_orm::ActiveValue::{NotSet, Set};

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "GameDataInput")]
pub struct GameDataType {
    game_id: i32,
    key: String,
    value: Option<String>,
}

impl GameDataType {
    pub fn sample() -> Self {
        Self {
            game_id: 1,
            key: "title".to_string(),
            value: Some("Vulcanist".to_string()),
        }
    }
}

impl From<game_data::Model> for GameDataType {
    fn from(model: game_data::Model) -> Self {
        Self {
            game_id: model.game_id,
            key: model.key,
            value: model.value,
        }
    }
}

pub fn new_game_data(input: GameDataType) -> game_data::ActiveModel {
    let mut new_data = game_data::ActiveModel {
        game_id: Set(input.game_id),
        key: Set(input.key),
        value: NotSet,
    };
    if let Some(value) = input.value {
        new_data.value = Set(Some(value));
    }
    new_data
}
