use anyhow::{anyhow, Result};
use reqwest::{header, Client};

use chrono::{DateTime, Datelike, TimeDelta, Timelike, Weekday};
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

    pub async fn new_time_entry(
        &self,
        created: String,
        time_for_complete: i64,
        ignore_weekends: bool,
    ) -> Result<()> {
        let completed_task_time =
            match DateTime::parse_from_str(&created, "%Y-%m-%dT%H:%M:%S%.3f%z") {
                Ok(time) => time,
                Err(err) => return Err(anyhow!("Failed to parse creation time: {}", err)),
            };
        let duration_to_complete = TimeDelta::try_seconds(time_for_complete).unwrap();
        let working_hours = TimeDelta::try_hours(8).unwrap();

        let assumed_start_time = completed_task_time - duration_to_complete;
        let slots_count = (duration_to_complete.num_seconds() as f64
            / working_hours.num_seconds() as f64)
            .ceil() as i32;

        let task_duration_seconds = duration_to_complete.num_seconds();
        let working_hours_seconds = working_hours.num_seconds();

        for i in 0..slots_count {
            let mut slot_start_time = assumed_start_time + working_hours * i as i32;

            if ignore_weekends {
                while slot_start_time.weekday() == Weekday::Sat
                    || slot_start_time.weekday() == Weekday::Sun
                {
                    slot_start_time = slot_start_time + TimeDelta::try_days(1).unwrap()
                }
            }

            let working_duration = TimeDelta::try_seconds(
                task_duration_seconds - i as i64 * working_hours_seconds as i64,
            )
            .unwrap();

            let mut slot_end_time =
                slot_start_time + std::cmp::min(working_hours, working_duration);

            // Check if slot starts before 9 AM or if it crosses into the next day
            if slot_start_time.time().hour() < 9
                || (slot_start_time.time().hour() == 9 && slot_start_time.time().minute() != 0)
            {
                let adjustment =
                    TimeDelta::try_hours((9 - slot_start_time.time().hour()).into()).unwrap();
                slot_start_time = slot_start_time + adjustment;
                slot_end_time = slot_start_time + std::cmp::min(working_hours, working_duration);
            }

            // Check if slot ends after 5 PM or if it crosses into the next day
            if slot_end_time.time().hour() > 17
                || (slot_end_time.time().hour() == 17 && slot_end_time.time().minute() != 0)
            {
                let adjustment =
                    TimeDelta::try_hours((slot_end_time.time().hour() - 17).into()).unwrap();
                slot_end_time = slot_end_time - adjustment;
                slot_start_time = slot_end_time - std::cmp::min(working_hours, working_duration);
            }

            let start_time_str = slot_start_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
            let end_time_str = slot_end_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();

            self.api_entry(start_time_str, end_time_str).await?;
        }

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
