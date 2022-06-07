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
        let user_id = ctx.data_opt::<String>();
        if user_id.is_none() {
            return Err(FieldError::new("Please sign-in with replit first."));
        }
        let user_id = user_id.unwrap().parse::<i32>().unwrap();
        if input.id != user_id {
            return Err(FieldError::new("Invalid request."));
        }
        let user = new_user(input).insert(db).await?;
        Ok(user.into())
    }
}
