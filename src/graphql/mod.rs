use async_graphql::EmptySubscription;
use async_graphql::Schema;

pub mod mutation;
pub mod query;
pub mod types;

pub type CarnivalSchema = Schema<query::Query, mutation::Mutation, EmptySubscription>;
