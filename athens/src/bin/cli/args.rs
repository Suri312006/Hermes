use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Creates an account and sets up your device.
    /// Will also lauch the Athens proxy to communicate on
    /// a regular interval with Sparta.
    Init {},

    #[command(subcommand)]
    Message(MessageSubCommands),

    Contacts(ContactsArgs),
}

#[derive(Subcommand, Debug)]
pub enum MessageSubCommands {
    Send {
        #[arg(short, long)]
        message: String,

        #[arg(short, long)]
        recipient: String,

        #[arg(short, long)]
        path: Option<String>,
    },
    Fetch,
}

#[derive(Debug, Args)]
pub struct ContactsArgs {}
