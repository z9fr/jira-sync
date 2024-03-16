use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueResponse {
    pub expand: String,
    pub start_at: i64,
    pub max_results: i64,
    pub total: i64,
    pub issues: Vec<Issue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub expand: String,
    pub id: String,
    #[serde(rename = "self")]
    pub self_field: String,
    pub key: String,
    pub fields: Fields,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub statuscategorychangedate: String,
    pub fix_versions: Vec<FixVersion>,
    pub resolution: Option<Resolution>,
    pub last_viewed: Option<String>,
    pub priority: Priority,
    pub labels: Vec<String>,
    pub aggregatetimeoriginalestimate: Option<i64>,
    pub timeestimate: Option<i64>,
    pub versions: Vec<Version>,
    pub issuelinks: Vec<Issuelink>,
    pub assignee: Assignee,
    pub status: Status3,
    pub components: Vec<Value>,
    pub aggregatetimeestimate: Option<i64>,
    pub creator: Creator,
    pub subtasks: Vec<Subtask>,
    pub reporter: Reporter,
    pub aggregateprogress: Aggregateprogress,
    pub progress: Progress,
    pub votes: Votes,
    pub issuetype: Issuetype4,
    pub timespent: Value,
    pub project: Project,
    pub aggregatetimespent: Option<i64>,
    pub resolutiondate: Option<String>,
    pub workratio: i64,
    pub watches: Watches,
    pub created: String,
    pub updated: String,
    pub timeoriginalestimate: Option<i64>,
    pub description: Option<Description>,
    pub security: Value,
    pub summary: String,
    pub environment: Value,
    pub duedate: Value,
    pub parent: Option<Parent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixVersion {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub description: String,
    pub name: String,
    pub archived: bool,
    pub released: bool,
    pub release_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub description: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    #[serde(rename = "self")]
    pub self_field: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub description: String,
    pub name: String,
    pub archived: bool,
    pub released: bool,
    pub release_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuelink {
    pub id: String,
    #[serde(rename = "self")]
    pub self_field: String,
    #[serde(rename = "type")]
    pub type_field: Type,
    pub inward_issue: Option<InwardIssue>,
    pub outward_issue: Option<OutwardIssue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub id: String,
    pub name: String,
    pub inward: String,
    pub outward: String,
    #[serde(rename = "self")]
    pub self_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InwardIssue {
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub self_field: String,
    pub fields: Fields2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields2 {
    pub summary: String,
    pub status: Status,
    pub priority: Priority2,
    pub issuetype: Issuetype,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "self")]
    pub self_field: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
    pub status_category: StatusCategory,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: i64,
    pub key: String,
    pub color_name: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority2 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuetype {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub subtask: bool,
    pub avatar_id: i64,
    pub hierarchy_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutwardIssue {
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub self_field: String,
    pub fields: Fields3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields3 {
    pub summary: String,
    pub status: Status2,
    pub priority: Priority3,
    pub issuetype: Issuetype2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status2 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
    pub status_category: StatusCategory2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory2 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: i64,
    pub key: String,
    pub color_name: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority3 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuetype2 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub subtask: bool,
    pub avatar_id: i64,
    pub hierarchy_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignee {
    #[serde(rename = "self")]
    pub self_field: String,
    pub account_id: String,
    pub email_address: String,
    pub avatar_urls: AvatarUrls,
    pub display_name: String,
    pub active: bool,
    pub time_zone: String,
    pub account_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarUrls {
    #[serde(rename = "48x48")]
    pub n48x48: String,
    #[serde(rename = "24x24")]
    pub n24x24: String,
    #[serde(rename = "16x16")]
    pub n16x16: String,
    #[serde(rename = "32x32")]
    pub n32x32: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status3 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
    pub status_category: StatusCategory3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory3 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: i64,
    pub key: String,
    pub color_name: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    #[serde(rename = "self")]
    pub self_field: String,
    pub account_id: String,
    pub email_address: String,
    pub avatar_urls: AvatarUrls2,
    pub display_name: String,
    pub active: bool,
    pub time_zone: String,
    pub account_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarUrls2 {
    #[serde(rename = "48x48")]
    pub n48x48: String,
    #[serde(rename = "24x24")]
    pub n24x24: String,
    #[serde(rename = "16x16")]
    pub n16x16: String,
    #[serde(rename = "32x32")]
    pub n32x32: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtask {
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub self_field: String,
    pub fields: Fields4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields4 {
    pub summary: String,
    pub status: Status4,
    pub priority: Priority4,
    pub issuetype: Issuetype3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status4 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
    pub status_category: StatusCategory4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory4 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: i64,
    pub key: String,
    pub color_name: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority4 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuetype3 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub subtask: bool,
    pub avatar_id: i64,
    pub hierarchy_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reporter {
    #[serde(rename = "self")]
    pub self_field: String,
    pub account_id: String,
    pub email_address: String,
    pub avatar_urls: AvatarUrls3,
    pub display_name: String,
    pub active: bool,
    pub time_zone: String,
    pub account_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarUrls3 {
    #[serde(rename = "48x48")]
    pub n48x48: String,
    #[serde(rename = "24x24")]
    pub n24x24: String,
    #[serde(rename = "16x16")]
    pub n16x16: String,
    #[serde(rename = "32x32")]
    pub n32x32: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregateprogress {
    pub progress: i64,
    pub total: i64,
    pub percent: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub progress: i64,
    pub total: i64,
    pub percent: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Votes {
    #[serde(rename = "self")]
    pub self_field: String,
    pub votes: i64,
    pub has_voted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuetype4 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub subtask: bool,
    pub avatar_id: i64,
    pub hierarchy_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub project_type_key: String,
    pub simplified: bool,
    pub avatar_urls: AvatarUrls4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarUrls4 {
    #[serde(rename = "48x48")]
    pub n48x48: String,
    #[serde(rename = "24x24")]
    pub n24x24: String,
    #[serde(rename = "16x16")]
    pub n16x16: String,
    #[serde(rename = "32x32")]
    pub n32x32: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watches {
    #[serde(rename = "self")]
    pub self_field: String,
    pub watch_count: i64,
    pub is_watching: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonEditableReason {
    pub reason: String,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: i64,
    pub content: Vec<Content>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<Content2>,
    pub attrs: Option<Attrs7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    #[serde(default)]
    pub marks: Vec<Mark>,
    #[serde(default)]
    pub content: Vec<Content3>,
    pub attrs: Option<Attrs6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mark {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attrs: Option<Attrs>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<Content4>,
    pub attrs: Option<Attrs5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content4 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    #[serde(default)]
    pub content: Vec<Content5>,
    #[serde(default)]
    pub marks: Vec<Mark3>,
    pub attrs: Option<Attrs4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content5 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub content: Vec<Content6>,
    pub text: Option<String>,
    pub marks: Option<Vec<Mark2>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content6 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mark2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attrs: Option<Attrs2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mark3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attrs: Option<Attrs3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs3 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs4 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs5 {
    pub order: Option<i64>,
    pub background: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs6 {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub id: Option<String>,
    pub collection: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub url: Option<String>,
    pub text: Option<String>,
    pub access_level: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs7 {
    pub order: Option<i64>,
    pub layout: Option<String>,
    pub width: Option<i64>,
    pub is_number_column_enabled: Option<bool>,
    pub local_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub self_field: String,
    pub fields: Fields5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields5 {
    pub summary: String,
    pub status: Status5,
    pub priority: Priority5,
    pub issuetype: Issuetype5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status5 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
    pub status_category: StatusCategory5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory5 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: i64,
    pub key: String,
    pub color_name: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority5 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuetype5 {
    #[serde(rename = "self")]
    pub self_field: String,
    pub id: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
    pub subtask: bool,
    pub avatar_id: i64,
    pub hierarchy_level: i64,
}
