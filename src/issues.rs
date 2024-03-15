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
    pub timeestimate: i64,
    pub versions: Vec<Value>,
    pub issuelinks: Vec<Issuelink>,
    pub assignee: Assignee,
    pub status: Status2,
    pub components: Vec<Value>,
    pub aggregatetimeestimate: i64,
    pub creator: Creator,
    pub subtasks: Vec<Value>,
    pub aggregateprogress: Aggregateprogress,
    pub progress: Progress,
    pub votes: Votes,
    pub issuetype: Issuetype2,
    pub timespent: i64,
    pub project: Project,
    pub aggregatetimespent: i64,
    pub resolutiondate: Option<String>,
    pub workratio: i64,
    pub watches: Watches,
    pub created: String,
    pub updated: String,
    pub timeoriginalestimate: Option<i64>,
    pub description: Option<Description>,
    pub security: Value,
    pub summary: String,
    pub environment: Option<Environment>,
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
pub struct Issuelink {
    pub id: String,
    #[serde(rename = "self")]
    pub self_field: String,
    #[serde(rename = "type")]
    pub type_field: Type,
    pub outward_issue: OutwardIssue,
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
pub struct OutwardIssue {
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
    pub percent: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub progress: i64,
    pub total: i64,
    pub percent: i64,
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
    pub version: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<Content>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<Content2>,
    pub attrs: Option<Attrs3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    pub attrs: Option<Attrs>,
    #[serde(default)]
    pub marks: Vec<Mark>,
    #[serde(default)]
    pub content: Vec<Content3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub collection: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mark {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attrs: Option<Attrs2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs2 {
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    #[serde(default)]
    pub content: Vec<Content4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content4 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    #[serde(default)]
    pub content: Vec<Content5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content5 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    #[serde(default)]
    pub content: Vec<Content6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content6 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<Content7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content7 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs3 {
    pub panel_type: Option<String>,
    pub layout: Option<String>,
    pub order: Option<i64>,
    pub level: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub version: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<Content8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content8 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<Content9>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content9 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
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
    pub status: Status3,
    pub priority: Priority3,
    pub issuetype: Issuetype3,
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
pub struct Priority3 {
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
