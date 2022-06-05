use crate::graphql::types::user::UserType;
use async_graphql::*;
use entity::user;
use entity::user::Entity as User;
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
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
        if users.len() > 0 {
            let user = users[0].clone().into();
            Ok(user)
        } else {
            Err(FieldError::new("User not found"))
        }
    }
    pub async fn test_user(&self) -> FieldResult<UserType> {
        Ok(UserType::sample())
    }
}
