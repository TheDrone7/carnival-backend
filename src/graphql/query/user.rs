use crate::graphql::types::user::UserType;
use async_graphql::*;
use entity::user;
use entity::user::Entity as User;
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn user<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> FieldResult<UserType> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let result: Option<user::Model> = User::find_by_id(id).one(db).await?;
        if result.is_some() {
            let user = result.unwrap().into();
            Ok(user)
        } else {
            Err(FieldError::new("Invalid ID, user not found."))
        }
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
        if !users.is_empty() {
            let user = users[0].clone().into();
            Ok(user)
        } else {
            Err(FieldError::new("User not found"))
        }
    }

    pub async fn current_user<'ctx>(&self, ctx: &Context<'ctx>) -> FieldResult<UserType> {
        let token = ctx.data_opt::<String>();
        if token.is_none() {
            return Err(FieldError::new("You are not logged in."));
        }
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let user_id = token.unwrap().parse::<i32>().unwrap();
        let result: Option<user::Model> = User::find_by_id(user_id).one(db).await?;
        if result.is_some() {
            let user = result.unwrap().into();
            Ok(user)
        } else {
            Err(FieldError::new("Invalid token, user not found."))
        }
    }

    pub async fn test_user(&self) -> FieldResult<UserType> {
        Ok(UserType::sample())
    }
}
