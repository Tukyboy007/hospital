use crate::error::error::AppError;
use crate::models::models::{NewPost, NewUser, Post, User};
use crate::schema::schema::{posts, users};
use diesel::prelude::*;

pub fn create_user(
    conn: &mut PgConnection,
    username: String,
    email: String,
) -> Result<User, AppError> {
    let new_user = NewUser { username, email };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .map_err(AppError::from)
}

pub fn create_post(
    conn: &mut PgConnection,
    user_id: i32,
    title: String,
    body: String,
    published: bool,
) -> Result<(), AppError> {
    let new_post = NewPost {
        user_id,
        title,
        body,
        published,
    };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)?;
    Ok(())
}

pub fn get_posts_with_users(conn: &mut PgConnection) -> Result<Vec<(Post, User)>, AppError> {
    posts::table
        .inner_join(users::table)
        .load::<(Post, User)>(conn)
        .map_err(AppError::from)
}
