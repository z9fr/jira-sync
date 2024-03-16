use std::env;

use anyhow::Result;
use jira_sync::clockify::Clockify;
use jira_sync::Jira;

#[tokio::main]
async fn main() -> Result<()> {
    println!("helo world");
    let api_key = env::var("CLOCKIFY_API_KEY").expect("api key not found");
    let host = "surgeglobal.atlassian.net";
    //let jira = Jira::new(host, &api_key, None);
    //let tasks = jira.tasks_to_csv(true).await?;

    let clockify = Clockify::new(
        &api_key,
        "61c963b360b5421863a9f4f9",
        "5c2eed9ab079874ebdcefc52",
        None,
    );
    clockify
        .new_time_entry(String::from("2024-03-13T16:31:37.379+0530"), 259200, true)
        .await?;

    Ok(())
}
