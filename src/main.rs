mod db;
mod models;
mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let conn = db::get_connection().expect("Erro ao conectar no banco");
    db::initilize_db(&conn).expect("Erro ao inicializar o banco");

    let state = Arc::new(Mutex::new(conn));

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/post/:id", get(routes::show_post))
        .route("/new", get(routes::new_post_form))
        .route("/new", post(routes::create_post))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Blog rodando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
