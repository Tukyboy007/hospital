use crate::AppError;
use crate::db::db::DbPool;
use crate::models::models::{NewPost, NewUser, Post, PostGetUser, User};
use crate::schema::schema::posts::dsl::*;
use crate::schema::schema::users::dsl::*;

use crate::schema::schema::{posts, users};

use actix_web::{HttpResponse, Responder, web};
use diesel::associations::HasTable;
use diesel::dsl::sql;
use diesel::prelude::*;

#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

pub async fn create_user_handler(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> Result<impl Responder, AppError> {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let user = diesel::insert_into(users)
        .values(&*new_user)
        .get_result::<User>(&mut conn)
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn create_post_handler(
    pool: web::Data<DbPool>,
    new_post: web::Json<NewPost>,
) -> Result<impl Responder, AppError> {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let post = diesel::insert_into(posts)
        .values(&*new_post)
        .get_result::<Post>(&mut conn)
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(post))
}

pub async fn get_posts_handler(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<impl Responder, AppError> {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let results = posts
        .inner_join(users)
        .limit(per_page as i64)
        .offset(offset as i64)
        .load::<(Post, User)>(&mut conn)
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn search_posts_handler(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
    search_term: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let like_pattern = format!("%{}%", search_term.as_str());
    let results = posts
        .inner_join(users)
        .filter(title.ilike(&like_pattern).or(body.ilike(&like_pattern)))
        .limit(per_page as i64)
        .offset(offset as i64)
        .load::<(Post, User)>(&mut conn)
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn full_text_search_posts_handler(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
    search_term: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let search_query = search_term.as_str().replace(" ", " & ");
    let results = posts
        .inner_join(users)
        .filter(
            sql::<diesel::sql_types::Bool>("search_vector @@ to_tsquery('english', ")
                .bind::<diesel::sql_types::Text, _>(search_query)
                .sql(")"),
        )
        .limit(per_page as i64)
        .offset(offset as i64)
        .load::<(Post, User)>(&mut conn)
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn select(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<impl Responder, AppError> {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let results = posts::table
        .inner_join(users::table)
        .select((posts::id, posts::title, users::username))
        .limit(per_page as i64)
        .offset(offset as i64)
        .load::<PostGetUser>(&mut conn)
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(results))
}
