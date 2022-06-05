use crate::graphql::types::user::{new_user, UserType};
use async_graphql::*;
use sea_orm::{entity::*, DatabaseConnection};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: UserType,
    ) -> FieldResult<UserType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let user = new_user(input).insert(db).await?;
        Ok(user.into())
    }
}
