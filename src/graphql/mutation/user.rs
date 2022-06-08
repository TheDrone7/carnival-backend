use crate::graphql::types::user::{new_user, UserConfig, UserType};
use async_graphql::*;
use entity::user::Entity as User;
use sea_orm::{entity::*, ActiveValue::Set, DatabaseConnection};

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

    pub async fn update_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i32,
        input: UserConfig,
    ) -> FieldResult<UserType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let user_id = ctx.data_opt::<String>();
        if user_id.is_none() {
            return Err(FieldError::new("Please sign-in with replit first."));
        }
        let user_id = user_id.unwrap().parse::<i32>().unwrap();
        if id != user_id {
            return Err(FieldError::new("Invalid request."));
        }
        let some_user = User::find_by_id(id).one(db).await?;
        if some_user.is_none() {
            return Err(FieldError::new("User not found."));
        }
        let mut some_user = some_user.unwrap().into_active_model();
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
}
