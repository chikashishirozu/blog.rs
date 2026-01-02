use askama::Template;
use crate::models::_entities::posts::Model;
// use crate::filters;

#[derive(Template)]
#[template(path = "posts/index.html")]
pub struct PostsIndex {
    pub posts: Vec<Model>,
}

#[derive(Template)]
#[template(path = "posts/show.html")]
pub struct PostShow {
    pub post: Model,
}

#[derive(Template)]
#[template(path = "posts/form.html")]
pub struct PostForm {
    pub title: String,
    pub md_content: String,
}
