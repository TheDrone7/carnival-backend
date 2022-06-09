pub mod game;
pub mod game_api_info;
pub mod game_data;
pub mod user;

use async_graphql::MergedObject;
use game::GameQuery;
use game_api_info::GameApiInfoQuery;
use game_data::GameDataQuery;
use user::UserQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, GameQuery, GameDataQuery, GameApiInfoQuery);
