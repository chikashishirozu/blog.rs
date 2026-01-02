// src/controllers/mod.rs
pub mod auth; 
pub mod post; 
pub mod posts_html; 
pub mod user; 

pub use auth::routes as auth_routes;
pub use post::routes as post_routes;
pub use user::routes as user_routes;
pub use posts_html::routes as html_routes;  // ← 必要なら追加

use loco_rs::prelude::*; 

pub fn html_routes() -> Routes {
    Routes::new()
//        .prefix("api")  // ← これを使うのが一番きれい
        .add("/posts", get(posts_html::index).post(posts_html::create))
        .add("/posts/new", get(posts_html::new_post))
        .add("/posts/:title", get(posts_html::show))
        .add("/posts/:title/edit", get(posts_html::edit).post(posts_html::update))
        .add("/posts/:title/delete", post(posts_html::delete_post))
}
