use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "jira-sync",
    author = "z9fr <z9fr@protonmail.com>",
    version = "1.0"
)]
#[command(
    about = r#"The "Jira-Clockify Sync" CLI utility is a powerful tool crafted to streamline the process of updating Jira 
    task details and seamlessly synchronizing them with Clockify, simplifying time management and enhancing productivity in project workflows."#,
    long_about = r#"The "Jira-Clockify Sync" utility is a straightforward tool designed to streamline the process of updating time spent on 
    Jira tasks and automatically synchronizing it with Clockify. This utility simplifies the task of managing and tracking time across both platforms, 
    enhancing productivity and accuracy in project management workflows."#
)]
pub struct Cli {
    /// sub commands availible
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Configure(ConfigureArgs),
    #[command(
        about = "exports jira tasks as a csv to be able to update the timelogs in one place"
    )]
    Download {
        /// file to write the csv output
        #[arg(value_name = "PATH", help = "the path of the csv to be exported")]
        path: String,
        /// jira api max limit
        #[arg(
            value_name = "LIMIT",
            default_value_t = 50,
            help = "total result limit for the csv"
        )]
        limit: i64,
        // if the timespend is avaible results or not
        #[arg(
            value_name = "TIME_SPEND",
            default_value_t = ShowTimeSpendIssuesOptions::Enable,
            help = "timespend is avaible results or not"
        )]
        time_spend_empty: ShowTimeSpendIssuesOptions,
    },
    #[command(
        about = "takes a csv file with updated timelogs and sync the tickets with clockify while updating the timelogs in jira"
    )]
    Sync {
        /// file to read the updated csv values
        #[arg(
            value_name = "PATH",
            help = "the path to the csv which contains the updated timelogs"
        )]
        path: String,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
#[command(
    about = "configure jira and clockify properties",
    long_about = r#"This command enables users to specify various properties necessary for the integration, such as API keys, project IDs,
                        and other configuration parameters. These properties are typically stored in a configuration file for ease of management and deployment."#
)]
pub struct ConfigureArgs {
    /// config sub commands availible
    #[command(subcommand)]
    pub command: Option<ConfigureCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ConfigureCommands {
    #[command(about = "initiate the configuration file")]
    Init {},
    #[command(about = "view the location for the current configuration file")]
    Config {},
    #[command(about = "update the existing jira api key")]
    JiraApi {
        #[arg(
            value_name = "JIRA_API",
            help = "jira api key, this is optional if the value is missing a password input will show by default"
        )]
        key: Option<String>,
    },
    #[command(about = "update the existing jira project name")]
    JiraProject {
        #[arg(value_name = "JIRA_PROJECT", help = "the name of the jira project")]
        key: String,
    },
    #[command(about = "update the existing clockify api key")]
    ClockifyApi {
        #[arg(
            value_name = "CLOCKIFY_API",
            help = "clockify api key, this is optional if the value is missing a password input will show by default"
        )]
        key: Option<String>,
    },
    #[command(about = "update the existing clockify workspace id")]
    ClockifyWorkspace {
        #[arg(value_name = "CLOCKIFY_WORKSPACE", help = "clockify workspace-id")]
        key: String,
    },
    #[command(about = "update the existing clockify project id you want the data to sync")]
    ClockifyProject {
        #[arg(value_name = "CLOCKIFY_PROJECT", help = "clockify project-id")]
        key: String,
    },
    #[command(about = "enable debug mode, this will print additional information")]
    SetDebug {
        #[arg(value_name = "DEBUG", help = "enable or dissable debug mode")]
        key: SetDebug,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SetDebug {
    Enable,
    Disable,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShowTimeSpendIssuesOptions {
    Enable,
    Disable,
}

impl std::fmt::Display for ShowTimeSpendIssuesOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
