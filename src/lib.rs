use anyhow::Result;
use csv::Writer;
use reqwest::{header, Client};
use std::fs::{self, File};

pub mod clockify;
mod issues;
mod jira_issues_result;

use crate::{issues::TransformedIssue, jira_issues_result::IssueResponse};

pub struct Jira {
    pub host: String,
    client: Client,
    base_url: String,
    debug: bool,
}

impl Jira {
    pub fn new(host: &str, api_key: &str, version: Option<&str>, debug: bool) -> Self {
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
            debug,
        }
    }

    pub async fn csv_to_tasks(self, path: &str) -> Result<()> {
        let file_content = fs::read_to_string(path).expect("Failed to read the file");
        let mut rdr = csv::Reader::from_reader(file_content.as_bytes());

        for result in rdr.deserialize() {
            let record: TransformedIssue = result?;

            println!("{:#?}", record);
        }

        Ok(())
    }

    pub async fn tasks_to_csv(
        self,
        file_name: &str,
        limit: i64,
        project_name: &str,
        timespent_is_empty: bool,
    ) -> Result<()> {
        let result = self
            .assigned_tasks(project_name, timespent_is_empty, limit)
            .await?;

        let file = File::create(file_name)?;
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

    pub async fn assigned_tasks(
        self,
        project_name: &str,
        timespent_is_empty: bool,
        limit: i64,
    ) -> Result<IssueResponse> {
        println!("{}", timespent_is_empty);

        let timespend_query = match timespent_is_empty {
            true => "timespent is empty",
            false => "timespent is not empty",
        };

        let jql = format!("project = \"{}\" AND assignee IN (currentUser()) AND statusCategory in (Done) AND {} ORDER BY created DESC", project_name, timespend_query);

        if self.debug {
            println!("generated jql {}", jql);
        }

        let params = [("jql", jql), ("maxResults", limit.to_string())];
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
