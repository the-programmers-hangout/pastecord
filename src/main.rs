use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_macros::debug_handler;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{error::Error, net::SocketAddr, sync::Arc};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use uuid::Uuid;

struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:secret@localhost/pastecord")
        .await
        .expect("Unable to connect to postgres");

    let state = AppState { db: pool };

    // build our application with a route
    let app = Router::new()
        .route("/documents", post(documents_post))
        .route("/documents/:id", get(documents_get))
        .nest_service(
            "/",
            ServeDir::new("static").fallback(ServeFile::new("static/index.html")),
        )
        .with_state(Arc::new(state))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct Created {
    key: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
struct Paste {
    id: Uuid,
    content: String,
    created_at: NaiveDateTime,
}

async fn add_paste(pool: &PgPool, content: String) -> Result<Uuid, Box<dyn Error>> {
    let rec = sqlx::query!(
        r#"
INSERT INTO pastes (id, content)
VALUES ( $1, $2 )
RETURNING id
        "#,
        Uuid::new_v4(),
        content,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

#[debug_handler]
async fn documents_post(State(state): State<Arc<AppState>>, body: String) -> impl IntoResponse {
    match add_paste(&state.db, body).await {
        Ok(created) => Json(Created {
            key: created.to_string(),
        })
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {}", e),
        )
            .into_response(),
    }
}

#[derive(Serialize)]
struct FoundPaste {
    key: Uuid,
    data: String,
}

async fn documents_get(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let item = sqlx::query_as!(Paste, "SELECT * FROM pastes WHERE id = $1", id)
        .fetch_one(&state.db)
        .await;

    match item {
        Ok(found) => Json(FoundPaste {
            data: found.content,
            key: found.id,
        })
        .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Paste not found").into_response(),
    }
}
