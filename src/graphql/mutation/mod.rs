pub mod game;
pub mod user;

use async_graphql::MergedObject;
use game::GameMutation;
use user::UserMutation;

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, GameMutation);
