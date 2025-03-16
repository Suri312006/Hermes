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

    Message(MessageArgs),

    Contacts(ContactsArgs),
}

#[derive(Debug, Args)]
pub struct MessageArgs {
    #[arg(short, long)]
    pub message: String,

    #[arg(short, long)]
    pub path: Option<String>,
}

#[derive(Debug, Args)]
pub struct ContactsArgs {}
