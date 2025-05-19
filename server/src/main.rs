use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use askama::Template;
use axum::{
    Json, Router, extract,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfigState {
    pub axum_database_url: String,
    pub axum_database_queries: PathBuf,
    pub axum_secret_key: String,
    pub axum_assets_file_dir: PathBuf,
    pub axum_assets_url: String,
    pub axum_host: String,
    pub axum_port: String,
}

impl AppConfigState {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok(); // Load from .env during dev
        envy::from_env()
    }
}

#[derive(Template)]
#[template(path = "partials/hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    assets_url: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

async fn greet(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

async fn index(State(state): State<Arc<AppConfigState>>) -> impl IntoResponse {
    let template = HomeTemplate {
        assets_url: state.axum_assets_url.clone(),
    };
    HtmlTemplate(template)
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
    msg: String,
}

// To Test the next function...
// curl -X POST http://127.0.0.1:3000/users \
//      -H "Content-Type: application/json" \
//      -d '{"username": "Timothy Nguyen"}'
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
        msg: String::from("yeah..."),
    };

    (StatusCode::CREATED, Json(user))
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppConfigState::from_env().expect("Failed to load configuration"));
    let static_files_dir = fs::canonicalize(&shared_state.axum_assets_file_dir)
        .expect("AXUM_ASSETS_FILE_DIR cannot be found");
    let _db_queries_dir = fs::canonicalize(&shared_state.axum_database_queries)
        .expect("AXUM_DATABASE_QUERIES cannot be found");

    let host = &shared_state.axum_host.clone();
    let port = &shared_state.axum_port.clone();
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await.expect(&addr);

    let app = Router::new()
        .nest_service("/public", ServeDir::new(static_files_dir))
        .route("/greet/{name}", get(greet))
        .route("/users", post(create_user))
        .route("/", get(index))
        .with_state(shared_state);

    println!("\nListening on {}\n", &addr);
    axum::serve(listener, app).await.unwrap();
}
