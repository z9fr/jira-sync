use std::env;

use anyhow::Result;
use jira_sync::Jira;

#[tokio::main]
async fn main() -> Result<()> {
    println!("helo world");
    let api_key = env::var("JIRA_API_KEY").unwrap();
    let host = "surgeglobal.atlassian.net";
    let jira = Jira::new(host, &api_key, None);

    let tasks = jira.assigned_tasks().await?;
    println!("{:?}", tasks);

    Ok(())
}
