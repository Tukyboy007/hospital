use crate::api::handlers::handlers::{
    create_post_handler, create_user_handler, full_text_search_posts_handler, get_posts_handler,
    search_posts_handler, select,
};
use actix_web::web::{self, ServiceConfig};

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/test", web::get().to(select))
            .route("/users", web::post().to(create_user_handler))
            .route("/posts", web::post().to(create_post_handler))
            .route("/posts", web::get().to(get_posts_handler))
            .route("/posts/search/{term}", web::get().to(search_posts_handler))
            .route(
                "/posts/full-text-search/{term}",
                web::get().to(full_text_search_posts_handler),
            ),
    );
}
