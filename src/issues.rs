use serde::{Deserialize, Serialize};

use crate::jira_issues_result::{Issue, IssueResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformedIssue {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(skip_serializing)]
    timeestimate: i64,
    #[serde(rename = "Time Estimate")]
    pub timeestimate_value: String,
    #[serde(skip_serializing)]
    timespent: i64,
    #[serde(rename = "Time Spent")]
    pub timespend_value: String,
    #[serde(rename = "Resolution Date")]
    pub resolutiondate: Option<String>,
    #[serde(rename = "Updated")]
    pub updated: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Summary")]
    pub summary: String,
    #[serde(rename = "Status Category Change Date")]
    pub statuscategorychangedate: String,
    #[serde(rename = "Self")]
    pub self_field: String,
}

impl From<Issue> for TransformedIssue {
    fn from(issue: Issue) -> Self {
        let timespent = issue.fields.timespent.as_i64().unwrap_or(0);
        let timeestimate = issue.fields.timeestimate.unwrap_or(0);
        TransformedIssue {
            id: issue.id,
            key: issue.key,
            timeestimate,
            timespent,
            timeestimate_value: Self::format_timespent(timeestimate),
            timespend_value: Self::format_timespent(timespent),
            resolutiondate: issue.fields.resolutiondate,
            updated: issue.fields.updated,
            created: issue.fields.created,
            summary: issue.fields.summary,
            statuscategorychangedate: issue.fields.statuscategorychangedate,
            self_field: issue.self_field,
        }
    }
}

impl TransformedIssue {
    pub fn format_timespent(timespent: i64) -> String {
        let days = timespent / (24 * 3600);
        let hours = (timespent % (24 * 3600)) / 3600;
        let minutes = (timespent % 3600) / 60;
        let seconds = timespent % 60;

        if days > 0 {
            format!("{}d", days)
        } else if hours > 0 {
            format!("{}h", hours)
        } else if minutes > 0 {
            format!("{}m", minutes)
        } else {
            format!("{}s", seconds)
        }
    }

    pub fn parse_timespent(self, time_str: &str) -> i64 {
        let mut parts = time_str.chars().peekable();
        let mut total_seconds = 0;
        let mut current_value = String::new();

        while let Some(c) = parts.next() {
            match c {
                'd' => total_seconds += current_value.parse::<i64>().unwrap_or(0) * 24 * 3600,
                'h' => total_seconds += current_value.parse::<i64>().unwrap_or(0) * 3600,
                'm' => total_seconds += current_value.parse::<i64>().unwrap_or(0) * 60,
                's' => total_seconds += current_value.parse::<i64>().unwrap_or(0),
                _ => {
                    current_value.push(c);
                    if parts.peek().is_none() {
                        total_seconds += current_value.parse::<i64>().unwrap_or(0);
                    }
                }
            }
        }

        total_seconds
    }
}

impl IssueResponse {
    pub fn map_issues_to_transformed(&self) -> Vec<TransformedIssue> {
        self.issues
            .iter()
            .map(|issue| issue.clone().into())
            .collect()
    }
}
