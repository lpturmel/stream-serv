use self::warcraftlogs::WarcraftLogs;
use axum::extract::State;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, Router};
use axum::Json;
use serde_json::json;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeFile;

pub mod warcraftlogs;

async fn get_progress(state: State<AppState>) -> Result<String, Error> {
    let progress_data = state.client.get_guild_progress().await.unwrap();

    let guild_data = progress_data
        .data
        .progress_race_data
        .progress_race
        .first()
        .ok_or(Error::GuildNotFound)?;
    let encounter_id = guild_data.current_encounter_id;
    let encounter = guild_data
        .encounters
        .iter()
        .find(|e| e.id == encounter_id)
        .ok_or(Error::EncounterNotFound)?;

    let res = format!(
        "Current progress: {}/{}M\n{}: {}%\n{} Pulls",
        guild_data.killed_count,
        guild_data.encounters.len(),
        encounter.short_name,
        guild_data.best_percent_of_non_killed_encounters,
        encounter.pull_count
    );
    Ok(res)
}

#[derive(Debug, Clone)]
struct AppState {
    client: Arc<WarcraftLogs>,
}

#[derive(Debug)]
enum Error {
    GuildNotFound,
    EncounterNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Error::GuildNotFound => (StatusCode::NOT_FOUND, "Guild not found"),
            Error::EncounterNotFound => (StatusCode::NOT_FOUND, "Encounter not found"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let warcraft_logs_client_id = std::env::var("WARCRAFT_LOGS_CLIENT_ID").unwrap();
    let warcraft_logs_client_secret = std::env::var("WARCRAFT_LOGS_CLIENT_SECRET").unwrap();
    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET]);
    let state = AppState {
        client: Arc::new(
            WarcraftLogs::init(&warcraft_logs_client_id, &warcraft_logs_client_secret).await,
        ),
    };
    let app = Router::new()
        .route_service("/index.html", ServeFile::new("assets/index.html"))
        .route("/progress", get(get_progress))
        .with_state(state)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3123")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
