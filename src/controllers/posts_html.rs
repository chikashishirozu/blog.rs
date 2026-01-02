use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    models::_entities::posts::{self, Entity},
    views::posts::{PostsIndex, PostShow, PostForm},
};

#[derive(Deserialize)]
pub struct PostFormData {
    pub title: String,
    pub md_content: String,
}

pub async fn index(State(ctx): State<AppContext>) -> Result<Html<String>> {
    let posts = Entity::find().all(&ctx.db).await?;
    let template = PostsIndex { posts };
    Ok(Html(template.render().map_err(|e| Error::string(&e.to_string()))?))
}

pub async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Form(form): Form<PostFormData>,
) -> Result<Response> {
    // 投稿作成ロジック
    let params = crate::models::posts::Params {
        title: form.title.clone(),
        md_content: form.md_content,
    };
    
    posts::Model::add(&ctx.db, &params, &auth.claims.pid).await?;
    
    Ok(Redirect::to("/posts").into_response())
}

pub async fn new_post(State(_ctx): State<AppContext>) -> Result<Html<String>> {
    // 新規作成フォームなので空のデータ
    let template = PostForm {
        title: String::new(),
        md_content: String::new()
    };
    Ok(Html(template.render().map_err(|e| Error::string(&e.to_string()))?))
}

pub async fn show(
    Path(title): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Html<String>> {
    // titleでpost検索(URLエンコードされたtitleを使用)
    let decoded_title = urlencoding::decode(&title)
        .map_err(|_| Error::BadRequest("Invalid title encoding".to_string()))?
        .to_string();
    
    let post_model = Entity::find()
        .filter(posts::Column::Title.eq(decoded_title))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    
    // PostShowテンプレート用に変換
    let post = posts::Model {
        id: post_model.id,
        title: post_model.title,
        md_content: post_model.md_content.or(Some(String::new())),
        user_id: post_model.user_id,
        created_at: post_model.created_at,
        updated_at: post_model.updated_at,
    };
    
    let template = PostShow { post };
    Ok(Html(template.render().map_err(|e| Error::string(&e.to_string()))?))
}

pub async fn edit(
    Path(title): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Html<String>> {
    // 編集フォーム表示
    let decoded_title = urlencoding::decode(&title)
        .map_err(|_| Error::BadRequest("Invalid title encoding".to_string()))?
        .to_string();
    
    let post = Entity::find()
        .filter(posts::Column::Title.eq(decoded_title))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    
    let template = PostForm {
        title: post.title,
        md_content: post.md_content.unwrap_or_default()
    };
    Ok(Html(template.render().map_err(|e| Error::string(&e.to_string()))?))
}

pub async fn update(
    auth: auth::JWT,
    Path(title): Path<String>,
    State(ctx): State<AppContext>,
    Form(form): Form<PostFormData>,
) -> Result<Response> {
    // 更新ロジック
    let decoded_title = urlencoding::decode(&title)
        .map_err(|_| Error::BadRequest("Invalid title encoding".to_string()))?
        .to_string();
    
    let post = Entity::find()
        .filter(posts::Column::Title.eq(decoded_title))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    
    let params = crate::models::posts::Params {
        title: form.title.clone(),
        md_content: form.md_content,
    };
    
    posts::Model::update(&ctx.db, post.id, &auth.claims.pid, &params).await?;
    
    // 新しいタイトルにリダイレクト
    let encoded_title = urlencoding::encode(&form.title);
    Ok(Redirect::to(&format!("/posts/{}", encoded_title)).into_response())
}

pub async fn delete_post(
    auth: auth::JWT,
    Path(title): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    // 削除ロジック
    let decoded_title = urlencoding::decode(&title)
        .map_err(|_| Error::BadRequest("Invalid title encoding".to_string()))?
        .to_string();
    
    let post = Entity::find()
        .filter(posts::Column::Title.eq(decoded_title))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    
    posts::Model::remove(&ctx.db, post.id, &auth.claims.pid).await?;
    
    Ok(Redirect::to("/posts").into_response())
}
