use std::env;
use std::process::exit;

use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};
use inquire::{error::InquireResult, min_length, Confirm, MultiSelect, Password, Select, Text};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;

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

#[derive(Debug, Parser)]
#[command(name = "jira-sync")]
#[command(about = "A cli to update jira tasks and sync clockify", long_about = Some("A simple utility to update Jira task timespend and automatically sync it with Clockify"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Configure(ConfigureArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct ConfigureArgs {
    #[command(subcommand)]
    command: Option<ConfigureCommands>,
}

#[derive(Debug, Subcommand)]
enum ConfigureCommands {
    Init {},
    Config {},
    JiraApi { key: Option<String> },
    JiraProject { key: String },
    ClockifyApi { key: Option<String> },
    ClockifyWorkspace { key: String },
    ClockifyProject { key: String },
    SetDebug { key: SetDebug },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum SetDebug {
    Enable,
    Disable,
}

lazy_static! {
    static ref APP_NAME: &'static str = "jira-sync";
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let mut config: AppConfig = confy::load(&APP_NAME, None)?;

    match args.command {
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
