pub mod game;
pub mod user;
use async_graphql::MergedObject;
use game::GameQuery;
use user::UserQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, GameQuery);
