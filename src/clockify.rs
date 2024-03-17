use anyhow::Result;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Clockify {
    client: Client,
    pub base_url: String,
    pub project_id: String,
    pub workspace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TimeEntry {
    pub start: String,
    pub end: String,
    pub billable: bool,
    pub description: String,
    pub project_id: String,
    pub task_id: Value,
    pub tag_ids: Vec<Value>,
    pub custom_fields: Vec<Value>,
}

impl Clockify {
    pub fn new(api_key: &str, project_id: &str, workspace_id: &str, url: Option<&str>) -> Self {
        let base_url = url.unwrap_or("https://api.clockify.me/api/v1");

        let client = Client::builder()
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::ACCEPT,
                    header::HeaderValue::from_static("application/json"),
                );
                headers.insert(header::USER_AGENT, "Clockify client".parse().unwrap());
                headers.insert(
                    "x-api-key",
                    header::HeaderValue::from_str(api_key).expect("Invalid api key"),
                );
                headers
            })
            .build()
            .expect("failed to create jira client");

        Self {
            client,
            project_id: project_id.to_string(),
            base_url: base_url.to_string(),
            workspace_id: workspace_id.to_string(),
        }
    }

    pub async fn new_time_entries(
        &self,
        start: String,
        end: String,
        description: &str,
        issue_key: &str,
    ) -> Result<()> {
        let time_entry = TimeEntry {
            project_id: self.project_id.to_string(),
            start,
            end,
            billable: true,
            description: format!("[{}] {}", issue_key, description).to_string(),
            task_id: Value::Null,
            tag_ids: Vec::new(),
            custom_fields: Vec::new(),
        };
        let url = format!(
            "{}/workspaces/{}/time-entries",
            self.base_url, self.workspace_id
        );

        let request = self
            .client
            .request(reqwest::Method::POST, url)
            .json(&time_entry)
            .send()
            .await?;

        let status = request.status();

        if !status.is_success() {
            let response = request.text().await?;
            println!("{:#?}", response);
        }

        Ok(())
    }
}
