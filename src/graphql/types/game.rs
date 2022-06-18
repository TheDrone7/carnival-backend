use crate::graphql::types::{game_api_info::GameApiInfoType, user::UserType};
use async_graphql::*;
use entity::{
    game, game_api_info::Entity as GameApiInfo, user::Entity as User, user::Model as UserModel,
};
use sea_orm::{entity::*, ActiveValue::Set, DatabaseConnection};

#[derive(InputObject)]
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

#[Object]
impl GameType {
    pub async fn id(&self) -> i32 {
        self.id
    }

    pub async fn title(&self) -> String {
        self.title.to_string()
    }

    pub async fn description(&self) -> String {
        self.description.to_string()
    }

    pub async fn repl_url(&self) -> String {
        self.repl_url.to_string()
    }

    pub async fn icon_url(&self) -> String {
        self.icon_url.to_string()
    }

    pub async fn cover_url(&self) -> String {
        self.cover_url.to_string()
    }

    pub async fn owner<'ctx>(&self, ctx: &'ctx Context<'_>) -> Option<UserType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let result = User::find_by_id(self.user_id).one(db).await;
        if let Ok(Some(result)) = result {
            return Some(result.into());
        }
        None
    }

    pub async fn api_info<'ctx>(&self, ctx: &Context<'ctx>) -> Option<GameApiInfoType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let current_user = ctx.data_unchecked::<Option<UserModel>>();
        if current_user.is_none() {
            return None;
        }
        let current_user = current_user.clone().unwrap();
        if self.user_id != current_user.id {
            return None;
        }
        let result = GameApiInfo::find_by_id(self.id).one(db).await;
        if let Ok(Some(result)) = result {
            return Some(result.into());
        }
        None
    }
}

impl GameType {
    pub fn sample() -> Self {
        Self {
            id: 1,
            title: "Vulcanist".to_string(),
            description: include_str!("sample_game_description.txt").to_string(),
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
    pub title: Option<String>,
    pub description: Option<String>,
    pub repl_url: Option<String>,
    pub icon_url: Option<String>,
    pub cover_url: Option<String>,
}
