use crate::graphql::types::user::UserType;
use async_graphql::*;
use entity::{user, user::Entity as User};
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn user<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> FieldResult<UserType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let result: Option<user::Model> = User::find_by_id(id).one(db).await?;
        if result.is_none() {
            return Err(FieldError::new("Invalid ID, user not found."));
        }
        Ok(result.unwrap().into())
    }

    pub async fn user_by_username<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> FieldResult<UserType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let users: Vec<user::Model> = User::find()
            .filter(user::Column::Username.eq(username))
            .order_by_asc(user::Column::Id)
            .all(db)
            .await
            .expect("Unable to fetch users");
        if users.is_empty() {
            return Err(FieldError::new("User not found"));
        }
        Ok(users[0].clone().into())
    }

    pub async fn current_user<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<UserType> {
        let user = ctx.data_unchecked::<Option<user::Model>>();
        if let Some(user) = user {
            return Ok(user.clone().into());
        }
        Err(FieldError::new("You are not logged in."))
    }

    pub async fn sample_user(&self) -> FieldResult<UserType> {
        Ok(UserType::sample())
    }
}
