use anyhow::Result;
use clap::Parser;
use inquire::{min_length, Password, Text};
use jira_sync::cli;
use jira_sync::cli::Cli;
use jira_sync::cli::Commands;
use jira_sync::cli::ConfigureCommands;
use jira_sync::cli::ShowTimeSpendIssuesOptions;
use jira_sync::clockify::Clockify;
use jira_sync::Jira;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::process::exit;

use crate::cli::SetDebug;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppConfig {
    jira_api_key: Option<String>,
    jira_host: String,
    jira_project_name: Option<String>,
    clockify_api_key: Option<String>,
    clockify_workspace: Option<String>,
    clockify_project_name: Option<String>,
    debug: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            jira_api_key: None,
            jira_host: "surgeglobal.atlassian.net".to_string(),
            jira_project_name: None,
            clockify_api_key: None,
            clockify_workspace: None,
            clockify_project_name: None,
            debug: false,
        }
    }
}

lazy_static! {
    static ref APP_NAME: &'static str = "jira-sync";
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let mut config: AppConfig = confy::load(&APP_NAME, None)?;

    match args.command {
        Commands::Sync { path } => {
            if fs::metadata(&path).is_err() {
                eprintln!("failed to open {}. make sure the file exist", path);
                exit(1);
            }

            let jira_key = match config.jira_api_key {
                Some(key) => key,
                None => {
                    eprintln!("Jira API key not found. Please configure it in the configuration.");
                    exit(1)
                }
            };

            let clockify_key = match config.clockify_api_key {
                Some(key) => key,
                None => {
                    eprintln!(
                        "Clockify API key not found. Please configure it in the configuration."
                    );
                    exit(1)
                }
            };
            let clockify_project_id = match config.clockify_project_name {
                Some(key) => key,
                None => {
                    eprintln!(
                        "Clockify Project not found. Please configure it in the configuration."
                    );
                    exit(1)
                }
            };
            let clockify_workspace_id = match config.clockify_workspace {
                Some(key) => key,
                None => {
                    eprintln!(
                        "Clockify workspace not found. Please configure it in the configuration."
                    );
                    exit(1)
                }
            };
            let jira = Jira::new(&config.jira_host, &jira_key, None, config.debug);
            let clockify = Clockify::new(
                &clockify_key,
                &clockify_project_id,
                &clockify_workspace_id,
                None,
            );
            jira.csv_to_tasks(&path, &clockify).await?;
        }
        Commands::Download {
            path,
            limit,
            time_spend_empty,
        } => {
            let jira_key = match config.jira_api_key {
                Some(key) => key,
                None => {
                    eprintln!("Jira API key not found. Please configure it in the configuration.");
                    exit(1)
                }
            };

            let project_name = match config.jira_project_name {
                Some(key) => key,
                None => {
                    eprintln!(
                        "Jira project name is missing, Please configure it in the configuration."
                    );
                    exit(1)
                }
            };

            let jira = Jira::new(&config.jira_host, &jira_key, None, config.debug);
            let timespent_is_empty = match time_spend_empty {
                ShowTimeSpendIssuesOptions::Enable => true,
                ShowTimeSpendIssuesOptions::Disable => false,
            };
            jira.tasks_to_csv(&path, limit, &project_name, timespent_is_empty)
                .await?;
            println!("Results saved as {} successfully.", path);
        }
        Commands::Configure(configure) => {
            let commands_cmd = configure.command.unwrap_or(ConfigureCommands::Init {});

            match commands_cmd {
                ConfigureCommands::Init {} => {
                    let jira_api_key = Password::new("Jira api key?")
                        .with_help_message(
                            "Your jira api key with the Basic Auth format, email:api-key as a base64 string"
                        )
                        .with_validator(min_length!(5, "Minimum of 5 characters")).prompt().expect("failed to read jira api key");

                    let jira_project = Text::new("Jira project name?")
                        .prompt()
                        .expect("failed to read project name");

                    let jira_host = Text::new("Jira host name")
                        .with_default("surgeglobal.atlassian.net")
                        .prompt_skippable()
                        .expect("failed to read jira hostname")
                        .unwrap();

                    let clockify_api_key = Password::new("Clockify api key?")
                        .prompt()
                        .expect("failed to read api key");
                    let clockify_workspace = Text::new("Clockify workspace id?")
                        .prompt()
                        .expect("failed to read workspace id");
                    let clockify_project_id = Text::new("Clockify project id?")
                        .prompt()
                        .expect("failed to read project id");

                    let config = AppConfig {
                        jira_api_key: Some(jira_api_key.clone()),
                        jira_project_name: Some(jira_project.clone()),
                        jira_host,
                        clockify_api_key: Some(clockify_api_key.clone()),
                        clockify_workspace: Some(clockify_workspace.clone()),
                        clockify_project_name: Some(clockify_project_id.clone()),
                        ..config.clone()
                    };

                    println!("{:?}", config);
                    confy::store(&APP_NAME, None, &config)?;
                    exit(0)
                }
                ConfigureCommands::Config {} => {
                    let config_file = confy::get_configuration_file_path(&APP_NAME, None)?;
                    println!("Config location: {:?}", config_file);
                }
                ConfigureCommands::JiraApi { key } => match key {
                    Some(x) => config.jira_api_key = Some(x),
                    None => {
                        let jira_api_key = Password::new("Jira api key?")
                        .with_help_message(
                            "Your jira api key with the Basic Auth format, email:api-key as a base64 string"
                        )
                        .with_validator(min_length!(5, "Minimum of 5 characters")).prompt().expect("failed to read jira api key");
                        config.jira_api_key = Some(jira_api_key)
                    }
                },
                ConfigureCommands::JiraProject { key } => config.jira_project_name = Some(key),
                ConfigureCommands::ClockifyApi { key } => match key {
                    Some(x) => config.clockify_api_key = Some(x),
                    None => {
                        let clockify_api_key = Password::new("Clockify api key?")
                            .prompt()
                            .expect("failed to read api key");

                        config.clockify_api_key = Some(clockify_api_key)
                    }
                },
                ConfigureCommands::ClockifyProject { key } => {
                    config.clockify_project_name = Some(key)
                }
                ConfigureCommands::ClockifyWorkspace { key } => {
                    config.clockify_workspace = Some(key)
                }
                ConfigureCommands::SetDebug { key } => match key {
                    SetDebug::Enable => config.debug = true,
                    SetDebug::Disable => config.debug = false,
                },
            }

            if config.debug {
                println!("{:#?}", config)
            }
            confy::store(&APP_NAME, None, &config)?;
        }
    }

    Ok(())
}
