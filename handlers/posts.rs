use axum::response::Html;

use crate::templates::posts::{
    PostIndexTemplate, PostShowTemplate, PostFormTemplate,
    PostSummary, PostDetail,
};

// 仮データ（後で DB と接続する）
fn sample_posts() -> Vec<PostSummary> {
    vec![
        PostSummary { id: 1, title: "First Post".into() },
        PostSummary { id: 2, title: "Second Post".into() },
    ]
}

pub async fn posts_index() -> Html<String> {
    let template = PostIndexTemplate {
        posts: sample_posts(),
    };

    Html(template.render().unwrap())
}

pub async fn posts_show() -> Html<String> {
    let post = PostDetail {
        id: 1,
        title: "First Post".to_string(),
        content: "This is the content.".to_string(),
    };

    let template = PostShowTemplate { post };

    Html(template.render().unwrap())
}

pub async fn posts_form() -> Html<String> {
    let template = PostFormTemplate { post: None };

    Html(template.render().unwrap())
}
