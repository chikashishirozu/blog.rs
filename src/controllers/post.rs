// src/controllers/post.rs
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::routing::patch;
use loco_rs::prelude::*;
use axum::extract::{State, Form};
use axum::response::{Html, Redirect};

use crate::models::{
    _entities::posts::{self, Entity, Model},
    posts::Params,
};

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    format::json(Entity::find().all(&ctx.db).await?)
}

// JSON形式でのPOST受付用
pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
    let user_id = auth.claims.pid;
    let item = posts::Model::add(&ctx.db, &params, &user_id).await?;
    format::json(item)
}

// フォームからのPOST受付用（現在は使用していない）
pub async fn create_form(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Form(params): Form<Params>,
) -> Result<Redirect> {
    let user_id = auth.claims.pid;
    posts::Model::add(&ctx.db, &params, &user_id).await?;
    Ok(Redirect::to("/posts"))
}

pub async fn update(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
    let update_post = posts::Model::update(&ctx.db, id, &auth.claims.pid, &params).await?;
    format::json(update_post)
}

pub async fn remove(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<()> {
    posts::Model::remove(&ctx.db, id, &auth.claims.pid).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<Model>> {
    format::json(load_item(&ctx, id).await?)
}

// 新しいフォーム表示関数
pub async fn new_form() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>新規投稿</title>
    <meta charset="utf-8">
    <style>
        body { font-family: Arial, sans-serif; max-width: 600px; margin: 50px auto; padding: 20px; }
        form { display: flex; flex-direction: column; gap: 15px; }
        input, textarea { padding: 8px; font-size: 16px; }
        button { padding: 10px; background-color: #007bff; color: white; border: none; cursor: pointer; }
        button:hover { background-color: #0056b3; }
    </style>
</head>
<body>
    <h1>新規投稿作成</h1>
    <form id="postForm">
        <input type="text" id="title" placeholder="タイトル" required>
        <textarea id="content" placeholder="内容" rows="10" required></textarea>
        <button type="submit">投稿する</button>
    </form>
    <script>
        document.getElementById('postForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            const title = document.getElementById('title').value;
            const content = document.getElementById('content').value;
            
            const response = await fetch('/api/posts', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ title, content })
            });
            
            if (response.ok) {
                alert('投稿が作成されました！');
                window.location.href = '/';
            } else {
                alert('エラーが発生しました');
            }
        });
    </script>
</body>
</html>
    "#)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("posts")
        .add("/", get(list))
        .add("/", post(add))  // これでPOST /api/postsが動作します
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", patch(update))
        .add("/new", get(new_form))
}
