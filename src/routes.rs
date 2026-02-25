use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    Form,
    extract::{Path, State},
    response::Redirect,
};
use rusqlite::Connection;
use std::sync::Arc;
use std::sync::Mutex;

use crate::db;
use crate::models::{NewPost, Post};

// O estado compartilhado da aplicação - a conexão com o banco
pub type AppState = Arc<Mutex<Connection>>;

// TEMPLATES

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    post: Post,
}

#[derive(Template)]
#[template(path = "new_post.html")]
struct NewPostTemplate;

// ROTAS

// GET / -> Página inicial com lista de posts
pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.lock().unwrap();
    let posts = db::get_all_posts(&conn).unwrap_or_default();
    IndexTemplate { posts }
}

// GET /post/:id -> página de um post individual
pub async fn show_post(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let conn = state.lock().unwrap();
    let post = db::get_post_by_id(&conn, id).unwrap();
    PostTemplate { post }
}

// GET /new -> formuário de novo post
pub async fn new_post_form() -> impl IntoResponse {
    NewPostTemplate
}

// POST /new -> salva o post e redireciona para o home
pub async fn create_post(State(state): State<AppState>, Form(new_post): Form<NewPost>) -> Redirect {
    let conn = state.lock().unwrap();
    db::create_post(&conn, &new_post).unwrap();
    Redirect::to("/")
}
