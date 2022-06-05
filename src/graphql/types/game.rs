use async_graphql::*;
use entity::game;
use sea_orm::ActiveValue::Set;

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "GameInput")]
pub struct GameType {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub repl_url: String,
    pub user_id: i32,
    pub icon_url: String,
    pub cover_url: String,
}

impl GameType {
    pub fn sample() -> Self {
        Self {
            id: 1,
            title: "Vulcanist".to_string(),
            description: ("Vulcanist is a first person shooter with seventeen levels, tons of different enemies\
            , three boss fights, and a story all but unexpected.  \
            Do you have what it takes to save your friend...or your life?\n\n---\nBe sure to check out my newest \
            project Replverse!\nhttps://replit.com/@IroncladDev/replverse\n---\n\nThe reason I named \
            it \"Vulcanist\" was because the player has a special flame attack that can only \
            be used so many times.\n\nGrab a pair of gaming headphones, get your keyboard ready, \
            and get your gaming skills up and running.  Get ready to play my best game.").to_string(),
            repl_url: "https://replit.com/@IroncladDev/Vulcanist".to_string(),
            user_id: 4150864,
            icon_url: "https://replit.com/public/images/languages/nodejs.svg".to_string(),
            cover_url: "https://storage.googleapis.com/replit/images/1636644190316_26884f8418097fcf6a7d57940fae6ed8.gif".to_string(),
        }
    }
}

impl From<game::Model> for GameType {
    fn from(model: game::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            description: model.description,
            repl_url: model.repl_url,
            user_id: model.user_id,
            icon_url: model.icon_url,
            cover_url: model.cover_url,
        }
    }
}

pub fn new_game(input: GameType) -> game::ActiveModel {
    game::ActiveModel {
        id: Set(input.id),
        title: Set(input.title),
        description: Set(input.description),
        repl_url: Set(input.repl_url),
        user_id: Set(input.user_id),
        icon_url: Set(input.icon_url),
        cover_url: Set(input.cover_url),
    }
}

#[derive(InputObject)]
pub struct GameConfig {
    id: Option<i32>,
    title: Option<String>,
    description: Option<String>,
    repl_url: Option<String>,
    user_id: Option<i32>,
    icon_url: Option<String>,
    cover_url: Option<String>,
}
