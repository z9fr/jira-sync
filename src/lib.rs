use std::fs::File;

use anyhow::Result;
use reqwest::{header, Client};

use csv::Writer;

pub mod clockify;
mod issues;
mod jira_issues_result;

use crate::{issues::TransformedIssue, jira_issues_result::IssueResponse};

pub struct Jira {
    pub host: String,
    client: Client,
    base_url: String,
}

impl Jira {
    pub fn new(host: &str, api_key: &str, version: Option<&str>) -> Self {
        let v = version.unwrap_or("3");
        let client = Client::builder()
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::ACCEPT,
                    header::HeaderValue::from_static("application/json"),
                );
                headers.insert(header::USER_AGENT, "Jira client".parse().unwrap());
                headers.insert(
                    header::AUTHORIZATION,
                    header::HeaderValue::from_str(&format!("Basic {}", api_key))
                        .expect("Invalid API Key"),
                );
                headers
            })
            .build()
            .expect("failed to create jira client");

        Self {
            host: host.to_string(),
            client,
            base_url: format!("https://{}/rest/api/{}", host, v),
        }
    }

    pub async fn tasks_to_csv(self, timespent_is_empty: bool) -> Result<()> {
        let result = self.assigned_tasks().await?;

        let file = File::create("transformed_issues.csv")?;
        let mut writer = Writer::from_writer(file);

        let _: Result<()> = result.issues.iter().try_for_each(|issue| {
            let transformed_issue = TransformedIssue::from(issue.clone());
            writer.serialize(transformed_issue)?;
            Ok(())
        });

        writer.flush()?;
        println!("CSV file created successfully.");
        Ok(())
    }

    pub async fn assigned_tasks(self) -> Result<IssueResponse> {
        let jql = "project = \"SLCFD\" AND assignee IN (currentUser()) AND statusCategory in (Done) AND timespent is empty ORDER BY created DESC";

        let params = [("jql", jql), ("maxResults", "50")];
        let url = reqwest::Url::parse_with_params(&format!("{}/search", self.base_url), &params)?;

        let request = self
            .client
            .request(reqwest::Method::GET, url)
            .send()
            .await?;

        let result = request.json::<IssueResponse>().await?;
        Ok(result)
    }
}
