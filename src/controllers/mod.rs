pub mod auth;
pub mod post;
pub mod posts_html;
pub mod user;

// 各モジュールのルート関数を再エクスポート
pub use auth::routes as auth_routes;
pub use post::routes as post_routes;
pub use user::routes as user_routes;
pub use posts_html::routes as html_routes;

// この関数は削除または修正が必要です
// 問題点: posts_html::index などの関数が見つからない
// pub fn html_routes() -> Router<AppContext> {
//    Router::new()
//        .nest("/user", user::routes())
//        .nest("/post", post::routes())
//        .nest("/auth", auth::routes())   
//        .nest("/posts_html", posts_html::routes())
        // ここで posts_html モジュールの関数を直接呼び出すか、
        // または posts_html::routes() を呼び出す
        // 今回は後者を使うので、この関数は削除して
        // 直接 posts_html::routes() を使うようにします
// }
