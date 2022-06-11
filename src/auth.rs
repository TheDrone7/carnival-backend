use actix_web::http::header::HeaderMap;
use entity::{
    game::Entity as Game,
    game::Model as GameModel,
    game_api_info::Entity as GameApiInfo,
    user::{Entity as User, Model as UserModel},
};
use sea_orm::{entity::*, DatabaseConnection};

pub async fn authenticate(headers: HeaderMap, db: &DatabaseConnection) -> Option<UserModel> {
    let user_id = headers.get("x-replit-user-id");
    if let Some(user_id) = user_id {
        if !user_id.is_empty() {
            let user_id = user_id.to_str().unwrap().parse::<i32>();
            if let Ok(user_id) = user_id {
                let user = User::find_by_id(user_id).one(db).await;
                if let Ok(Some(user)) = user {
                    return Some(user);
                }
            }
        }
    }
    None
}

pub async fn check_api_key(headers: HeaderMap, db: &DatabaseConnection) -> Option<GameModel> {
    let game_token = headers.get("x-carnival-api-token");
    let game_token = game_token.map(|tkn| tkn.to_str().unwrap().to_string());
    if let Some(game_token) = game_token {
        let result = base64::decode(game_token).unwrap();
        let result = std::str::from_utf8(result.as_slice()).unwrap().to_string();
        let parts: Vec<&str> = result.split(':').collect();
        if parts.len() == 2 {
            let game_id = parts[0].parse::<i32>();
            if let Ok(game_id) = game_id {
                let game_secret = parts[1].to_string();
                let game_api = GameApiInfo::find_by_id(game_id).one(db).await;
                let game = Game::find_by_id(game_id).one(db).await;
                if let Ok(Some(game_api)) = game_api {
                    if let Ok(Some(game)) = game {
                        if game_api.game_secret == game_secret {
                            return Some(game);
                        }
                    }
                }
            }
        }
    }
    None
}
