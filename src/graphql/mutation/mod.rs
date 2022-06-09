pub mod game;
pub mod game_api_info;
pub mod game_data;
pub mod user;

use async_graphql::MergedObject;
use game::GameMutation;
use game_api_info::GameApiInfoMutation;
use game_data::GameDataMutation;
use user::UserMutation;

#[derive(MergedObject, Default)]
pub struct Mutation(
    UserMutation,
    GameMutation,
    GameDataMutation,
    GameApiInfoMutation,
);
