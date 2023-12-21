use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct WarcraftLogs {
    client: reqwest::Client,
    access_token: String,
}

impl WarcraftLogs {
    pub async fn init(client_id: &str, client_secret: &str) -> Self {
        let client = reqwest::Client::new();
        let access_token = Self::get_access_token(client_id, client_secret).await;
        Self {
            client,
            access_token,
        }
    }

    async fn get_access_token(client_id: &str, client_secret: &str) -> String {
        let client = reqwest::Client::new();
        let response = client
            .post("https://www.warcraftlogs.com/oauth/token")
            .basic_auth(client_id, Some(client_secret))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await
            .unwrap();
        let json = response.json::<LoginResponse>().await.unwrap();
        json.access_token
    }
    pub async fn get_guild_progress(&self) -> Result<GuildProgressResponse, reqwest::Error> {
        let body = serde_json::json!({
            "query": r#"
            query {
                progressRaceData {
                    progressRace(guildID: 630742)
                }
            }
        "#
        });
        let response = self
            .client
            .post("https://www.warcraftlogs.com/api/v2/client")
            .header("Content-Type", "application/json")
            .bearer_auth(&self.access_token)
            .json(&body)
            .send()
            .await?;
        response.json::<GuildProgressResponse>().await
    }
}

#[derive(Debug, Deserialize)]
struct LoginResponse {
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildProgressResponse {
    pub data: ProgressRaceData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressRaceData {
    pub progress_race_data: ProgressRace,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressRace {
    pub progress_race: Vec<Guild>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    pub id: i64,
    pub name: String,
    pub faction: i64,
    pub logo_image_url: String,
    pub logo_image_is_custom: bool,
    pub logo_image_fallback_url: String,
    pub stream_channel: Value,
    pub rank: Value,
    pub killed_count: i64,
    pub name_css_class: String,
    pub rank_css_class: String,
    pub encounters: Vec<Encounter>,
    pub current_encounter_id: i64,
    pub coach: Value,
    pub guild_is_streaming: bool,
    pub last_kill_time: i64,
    pub best_percent_of_non_killed_encounters: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub id: i64,
    pub name: String,
    pub short_name: String,
    pub background_image_url: String,
    pub background_image_fallback_url: String,
    pub icon_image_url: String,
    pub transparent_image_url: String,
    pub is_killed: bool,
    pub killed_at_timestamp: Option<i64>,
    pub youtube_embed_url: Value,
    pub show_stats: bool,
    pub best_percent: f64,
    pub best_percent_for_display: String,
    pub pull_count: i64,
    pub per_pull: Vec<PerPull>,
    pub best_phase_index: i64,
    pub show_analyze_all_pulls_button: bool,
    pub should_show_fight_summary_charts: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerPull {
    pub report_code: String,
    pub fight_id: i64,
    pub report_is_private: bool,
    pub start_time: i64,
    pub end_time: i64,
    pub duration: i64,
    pub fight_percentage: f64,
    pub best_percent_for_display: String,
}
