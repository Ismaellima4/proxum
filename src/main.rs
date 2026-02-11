use crate::persitence::Repository;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod persitence;
mod protobuf;
mod user;
mod validate;

pub mod user_gen {
    include!(concat!(env!("OUT_DIR"), "/proxum.user.rs"));
}

pub type AppState = Arc<Repository>;

#[tokio::main]
async fn main() {
    tracing_config();
    let db_conn_url = std::env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://user:password@localhost/users_db"));

    let database_pool_size: u32 = std::env::var("DATABASE_POOL")
        .ok()
        .and_then(|pool_size| pool_size.parse().ok())
        .unwrap_or(30);

    let repository = Repository::connect(&db_conn_url, database_pool_size)
        .await
        .unwrap();

    let state = Arc::new(repository);
    let app = app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/users", post(handlers::save))
        .route(
            "/users/{id}",
            get(handlers::find_user_by_id).delete(handlers::delete_by_id),
        )
        .with_state(state)
}

fn tracing_config() {
    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init();
}
