use actix_web::http::header::HeaderMap;
use aes_gcm::{
    aead::{Aead, NewAead},
    Aes256Gcm, Key, Nonce,
};
use entity::{
    game::Entity as Game,
    game::Model as GameModel,
    game_api_info::Entity as GameApiInfo,
    user::{Entity as User, Model as UserModel},
};
use log::{error, warn};
use sea_orm::{entity::*, DatabaseConnection};
use std::env;

pub async fn authenticate(
    headers: HeaderMap,
    db: &DatabaseConnection,
) -> (Option<i32>, Option<UserModel>) {
    let pass = env::vars().find(|k| k.0 == "ENCRYPTION_KEY");
    if pass.is_none() {
        error!("No encryption key found");
        return (None, None);
    };
    let nonce = env::vars().find(|k| k.0 == "ENCRYPTION_NONCE");
    if nonce.is_none() {
        error!("No encryption nonce found");
        return (None, None);
    };
    let password = pass.unwrap().1;
    let iv = nonce.unwrap().1;
    let user_id = headers.get("x-carnival-user-token");
    if let Some(user_id) = user_id {
        let user_id = user_id.to_str().unwrap();
        let key = Key::from_slice(password.as_bytes());
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(iv.as_bytes());

        let decoded = base64::decode(user_id);
        if let Err(e) = decoded {
            warn!("Error decoding user token: {}", e);
            return (None, None);
        }
        let decoded = decoded.unwrap();
        let plaintext = cipher.decrypt(nonce, decoded.as_ref());
        if let Err(e) = plaintext {
            warn!("Error decrypting user token: {}", e);
            return (None, None);
        }
        let plaintext = plaintext.unwrap();
        let user_id = String::from_utf8(plaintext).expect("Failed to convert to string");
        let user_id = user_id.parse::<i32>();
        if let Err(e) = user_id {
            warn!("Error parsing user ID: {}", e);
            return (None, None);
        }
        let user_id = user_id.unwrap();
        let some_user = User::find_by_id(user_id).one(db).await;
        if let Ok(Some(some_user)) = some_user {
            return (Some(user_id), Some(some_user));
        } else {
            return (Some(user_id), None);
        }
    }
    (None, None)
}

pub async fn check_api_key(headers: HeaderMap, db: &DatabaseConnection) -> Option<GameModel> {
    let game_token = headers.get("x-carnival-api-token");
    let game_token = game_token.map(|tkn| tkn.to_str().unwrap().to_string());
    if let Some(game_token) = game_token {
        let result = base64::decode(game_token).unwrap();
        let result = std::str::from_utf8(result.as_slice()).unwrap().to_string();
        let parts: Vec<&str> = result.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        let game_id = parts[0].parse::<i32>();
        let game_secret = parts[1].to_string();
        if let Ok(game_id) = game_id {
            let game_api = GameApiInfo::find_by_id(game_id).one(db).await;
            let game = Game::find_by_id(game_id).one(db).await;
            if let (Ok(Some(game_api)), Ok(Some(game))) = (game_api, game) {
                if game_api.game_secret == game_secret {
                    return Some(game);
                }
            }
        }
    }
    None
}
