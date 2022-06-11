use crate::graphql::types::user::{new_user, UserConfig, UserType};
use async_graphql::*;
use entity::user::Model as UserModel;
use sea_orm::{entity::*, ActiveValue::Set, DatabaseConnection, DeleteResult};

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
        let user_id = ctx.data_opt::<i32>();
        if user_id.is_none() {
            return Err(FieldError::new("Please sign-in with replit first."));
        }
        let user_id: i32 = user_id.clone().unwrap().to_owned();
        if input.id != user_id {
            return Err(FieldError::new("Invalid request."));
        }
        let user = new_user(input).insert(db).await?;
        Ok(user.into())
    }

    pub async fn update_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i32,
        input: UserConfig,
    ) -> FieldResult<UserType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let user_id = ctx.data_opt::<UserModel>();
        if user_id.is_none() {
            return Err(FieldError::new("Please sign-in with replit first."));
        }
        let some_user = user_id.unwrap();
        let user_id = some_user.id;
        let mut some_user = some_user.clone().into_active_model();
        if id != user_id {
            return Err(FieldError::new("Invalid request."));
        }
        if input.username.is_some() {
            some_user.username = Set(input.username.unwrap());
        }
        if input.avatar_url.is_some() {
            some_user.avatar_url = Set(input.avatar_url.unwrap());
        }
        if input.full_name.is_some() {
            some_user.full_name = Set(input.full_name.unwrap());
        }
        if input.bio.is_some() {
            some_user.bio = Set(input.bio.unwrap());
        }
        if input.is_hacker.is_some() {
            some_user.is_hacker = Set(input.is_hacker.unwrap());
        }
        let result_user = some_user.update(db).await?;
        Ok(result_user.into())
    }

    pub async fn delete_user<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> FieldResult<UserType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let some_user = ctx.data_opt::<UserModel>();
        if some_user.is_none() {
            return Err(FieldError::new("Please sign-in with replit first."));
        }
        let some_user = some_user.unwrap();
        let user_id = some_user.id;
        if id != user_id {
            return Err(FieldError::new("Invalid request."));
        }
        let res: DeleteResult = some_user.clone().delete(db).await?;
        if res.rows_affected < 1 {
            return Err(FieldError::new("Unable to delete user."));
        }
        Ok(some_user.clone().into())
    }
}
