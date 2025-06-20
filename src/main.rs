use axum::{
    routing::{post}, Extension, Router
};
use sqlx::postgres::PgPoolOptions;
use tower_http::{trace::TraceLayer};
use std::{env, sync::Arc};
use zenv::Zenv;

mod handlers;
mod models;
mod repositories;
mod utils;

#[tokio::main]
async fn main() {
    Zenv::new(".env", false).configure().ok();
    let url = env::var("DATABASE_URL").expect("DATSBASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .unwrap();

    let auth_repository =  Arc::new(repositories::AuthRepository::new(pool));

    let app = Router::new()
        .route("/login", post(handlers::login_handler))
        .route("/registration", post(handlers::registration_handler))
        .layer(Extension(auth_repository))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
