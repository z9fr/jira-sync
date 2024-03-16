use anyhow::{anyhow, Result};
use reqwest::{header, Client};

use chrono::{DateTime, FixedOffset, TimeDelta};
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

    pub async fn new_time_entry(&self, created: String, seconds: i64) -> Result<()> {
        let working_hours = TimeDelta::try_hours(8).unwrap();
        let create_time = match DateTime::parse_from_str(&created, "%Y-%m-%dT%H:%M:%S%.3f%z") {
            Ok(time) => time,
            Err(err) => return Err(anyhow!("Failed to parse creation time: {}", err)),
        };

        let duration_to_complete = TimeDelta::try_seconds(seconds).unwrap();
        let complete_time_for_task = duration_to_complete / (8 * 3600);

        let start_time = create_time
            .checked_sub_signed(complete_time_for_task)
            .unwrap();
        let mut working_ranges = Vec::new();

        let mut current_time = start_time;

        while current_time < create_time {
            let end_time = (current_time + working_hours).min(create_time);
            working_ranges.push((current_time, end_time));
            current_time = end_time;
        }

        println!("{:?}", working_ranges);

        /*

        for (start, end) in working_ranges {
            let start_time_str = start.format("%Y-%m-%dT%H:%M:%SZ").to_string();
            let create_time_str = end.format("%Y-%m-%dT%H:%M:%SZ").to_string();

            self.api_entry(start_time_str, create_time_str).await?;
        }*/

        Ok(())
    }

    async fn api_entry(&self, start: String, end: String) -> Result<()> {
        let time_entry = TimeEntry {
            project_id: self.project_id.to_string(),
            start,
            end,
            billable: true,
            description: String::from("plugin testin"),
            task_id: Value::Null,
            tag_ids: Vec::new(),
            custom_fields: Vec::new(),
        };
        let url = format!(
            "{}/workspaces/{}/time-entries",
            self.base_url, self.workspace_id
        );

        println!("{:?}", time_entry);
        println!("{}", url);

        let request = self
            .client
            .request(reqwest::Method::POST, url)
            .json(&time_entry)
            .send()
            .await?;

        let response = request.text().await?;
        println!("{}", response);

        Ok(())
    }
}
