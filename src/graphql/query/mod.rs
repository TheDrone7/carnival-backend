pub mod game;
pub mod user;
pub mod game_data;
use async_graphql::MergedObject;
use game::GameQuery;
use user::UserQuery;
use game_data::GameDataQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, GameQuery, GameDataQuery);
