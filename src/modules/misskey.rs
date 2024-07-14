use reqwest::{Client, Result};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};

#[derive(Debug)]
pub struct MisskeyApiConfig {
    pub base_url: String,
    pub token: String,
}

pub struct MisskeyApi<'a> {
    config: &'a MisskeyApiConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct User {
    pub id: String,
    pub pinnedNoteIds: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Note {
    pub id: String,
    pub text: String,
    pub createdAt: String,
    pub myReaction: Option<String>,
}

pub struct ListNotesRequestParams {
    pub user_id: String,
    pub since_date: DateTime<FixedOffset>,
    pub limit: usize,
}

#[derive(Serialize, Debug)]
struct ListNotesReqBody {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "sinceDate")]
    pub since_date: i64,
    pub limit: usize,
    pub i: String,
}

impl ListNotesReqBody {
    fn new(params: ListNotesRequestParams, token: &String) -> Self {
        ListNotesReqBody {
            user_id: params.user_id,
            since_date: params.since_date.timestamp_millis(),
            limit: params.limit,
            i: token.clone(),
        }
    }
}

impl<'a> MisskeyApi<'a> {
    pub fn new(config: &'a MisskeyApiConfig) -> Self {
        MisskeyApi { config }
    }
}

impl MisskeyApi<'_> {
    pub async fn i(&self) -> Result<User> {
        let url = format!("{}/i", &self.config.base_url);
        let req_json = serde_json::json!({
            "i": &self.config.token,
        });
        let res = Client::new().post(&url).json(&req_json).send().await?;
        if !res.status().is_success() {
            panic!("Failed to fetch user.");
        }
        let json_body = res.json::<User>().await?;

        Ok(json_body)
    }

    pub async fn user_notes(&self, params: ListNotesRequestParams) -> Result<Vec<Note>> {
        let url = format!("{}/users/notes", &self.config.base_url);
        let req_body = ListNotesReqBody::new(params, &self.config.token);
        println!("{:?}", req_body);
        let req_json = serde_json::json!(req_body);
        let res = Client::new().post(&url).json(&req_json).send().await?;
        if !res.status().is_success() {
            panic!("Failed to fetch user notes.")
        }
        let json_body = res.json::<Vec<Note>>().await?;

        Ok(json_body)
    }

    pub async fn delete_note(&self, id: &String) -> Result<()> {
        let url = format!("{}/notes/delete", &self.config.base_url);
        let req_json = serde_json::json!({
            "noteId": id,
            "i": &self.config.token,
        });
        let res = Client::new().post(&url).json(&req_json).send().await?;
        if !res.status().is_success() {
            panic!("Failed to delete note.");
        }

        Ok(())
    }
}

