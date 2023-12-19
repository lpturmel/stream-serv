use axum::http::{HeaderValue, Method};
use axum::routing::{get, Router};
use tokio::process::Command;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeFile;

async fn get_progress() -> String {
    let cmd = Command::new("rio")
        .arg("guild")
        .arg("progress")
        .arg("--name")
        .arg("Chemical Imbalance")
        .arg("--boss")
        .arg("nymue")
        .arg("-d")
        .arg("mythic")
        .arg("--region")
        .arg("us")
        .arg("--realm")
        .arg("illidan")
        .arg("--raid")
        .arg("amirdrassil")
        .output()
        .await
        .unwrap();
    String::from_utf8(cmd.stdout).unwrap()
}
#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET]);
    let app = Router::new()
        .route("/progress", get(get_progress))
        .route_service("/index.html", ServeFile::new("assets/index.html"))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3123")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
