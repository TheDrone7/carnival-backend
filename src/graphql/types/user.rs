use crate::graphql::types::game::GameType;
use async_graphql::connection::*;
use async_graphql::*;
use entity::{game, user};
use sea_orm::{entity::*, query::*, ActiveValue::Set, DatabaseConnection};

#[derive(InputObject)]
#[graphql(input_name = "UserInput")]
pub struct UserType {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
    pub full_name: String,
    pub bio: String,
    pub is_hacker: bool,
}

pub struct UserGameConnection;
impl ConnectionNameType for UserGameConnection {
    fn type_name<T: OutputType>() -> String {
        "UserGameConnection".to_string()
    }
}

pub struct UserGameEdge;

impl EdgeNameType for UserGameEdge {
    fn type_name<T: OutputType>() -> String {
        "UserGameEdge".to_string()
    }
}

#[Object]
impl UserType {
    pub async fn id(&self) -> i32 {
        self.id
    }

    pub async fn username(&self) -> String {
        self.username.to_string()
    }

    pub async fn avatar_url(&self) -> String {
        self.avatar_url.to_string()
    }

    pub async fn full_name(&self) -> String {
        self.full_name.to_string()
    }

    pub async fn bio(&self) -> String {
        self.bio.to_string()
    }

    pub async fn is_hacker(&self) -> bool {
        self.is_hacker
    }

    pub async fn games<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Connection<i32, GameType, EmptyFields, EmptyFields, UserGameConnection, UserGameEdge> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let game_count: i32 = game::Entity::find()
            .filter(game::Column::UserId.eq(self.id))
            .order_by_asc(game::Column::Id)
            .count(db)
            .await
            .expect("Failed to get number of games") as i32;

        let mut start: i32 = match after {
            Some(after) => after.parse().unwrap(),
            None => 0,
        };
        let mut end: i32 = match before {
            Some(before) => before.parse().unwrap(),
            None => game_count,
        };
        if let Some(first) = first {
            end = (start + first as i32).min(end);
        }
        if let Some(last) = last {
            let last = last as i32;
            start = if last > end - start { end } else { end - last };
        }
        let mut conn = Connection::new(start > 0, end < game_count);

        let games: Vec<game::Model> = game::Entity::find()
            .filter(game::Column::UserId.eq(self.id))
            .filter(game::Column::Id.between(start, end))
            .order_by_asc(game::Column::Id)
            .all(db)
            .await
            .expect("Unable to fetch games");

        let mut games = games
            .into_iter()
            .map(|g| g.into())
            .map(|g: GameType| Edge::new(g.id, g))
            .collect();
        conn.edges.append(&mut games);

        conn
    }
}

impl UserType {
    pub fn sample() -> Self {
        UserType {
            id: 4150864,
            username: "IroncladDev".to_string(),
            avatar_url: "https://storage.googleapis.com/replit/images/1640002473076_f2d356e46577519b1463a26769c41e38.png".to_string(),
            full_name: "IroncladDev ⠕".to_string(),
            bio: "Replit site moderator | Sixteen-year-old fullstack developer | https://www.connerow.dev".to_string(),
            is_hacker: true
        }
    }
}

impl From<user::Model> for UserType {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            avatar_url: model.avatar_url,
            full_name: model.full_name,
            bio: model.bio,
            is_hacker: model.is_hacker,
        }
    }
}

pub fn new_user(user_input: UserType) -> user::ActiveModel {
    user::ActiveModel {
        id: Set(user_input.id),
        username: Set(user_input.username),
        avatar_url: Set(user_input.avatar_url),
        full_name: Set(user_input.full_name),
        bio: Set(user_input.bio),
        is_hacker: Set(user_input.is_hacker),
    }
}

#[derive(InputObject)]
pub struct UserConfig {
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub is_hacker: Option<bool>,
}
