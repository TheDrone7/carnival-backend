use async_graphql::*;
use base64::encode;
use entity::game_api_info;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sea_orm::ActiveValue::Set;
use std::str;

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "GameInput")]
pub struct GameApiInfoType {
    pub game_id: i32,
    pub game_secret: String,
    pub game_token: String,
}

impl GameApiInfoType {
    pub fn sample() -> Self {
        Self {
            game_id: 1,
            game_secret: "secret".to_string(),
            game_token: "token".to_string(),
        }
    }
}

impl From<game_api_info::Model> for GameApiInfoType {
    fn from(model: game_api_info::Model) -> Self {
        Self {
            game_id: model.game_id,
            game_secret: model.game_secret,
            game_token: model.game_token,
        }
    }
}

pub fn new_game_api_info(id: i32) -> game_api_info::ActiveModel {
    let secret: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(16).collect();
    let secret = str::from_utf8(secret.as_slice()).unwrap().to_string();
    let token = encode(id.to_string() + ":" + &secret);
    game_api_info::ActiveModel {
        game_id: Set(id),
        game_secret: Set(secret),
        game_token: Set(token),
    }
}
