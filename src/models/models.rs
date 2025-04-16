use crate::schema::schema::{posts, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Serialize, Queryable)]
pub struct PostGetUser {
    post_id: i32,
    title: String,
    username: String,
}
