use std::{
    ffi::{OsString},
};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run all startup commands
    Startup,
    /// Start a command in tmux
    Start { command: OsString },
    /// Add command to start
    Add {
        #[clap(flatten)]
        command: Command,
        /// Overwrite existing commands
        #[clap(short, long)]
        overwrite: bool,
    },
    /// Remove startup command
    Remove { name: OsString },
    /// Stop running programs
    Stop {
        #[clap(short, long)]
        /// Stop all registered processes
        all: bool,
        #[clap(short, long)]
        /// Immediately kill the window
        force: bool,
        /// Name of the command. Ignored if -a is specified.
        name: Option<OsString>,
    },
    /// List startable tmux windows
    List,
    /// Check if a command is running
    CheckRunning {
        #[clap(short, long)]
        /// Check all registered processes
        all: bool,
        /// Name of the command. Ignored if -a is specified.
        name: Option<OsString>,
    },
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Command {
    /// Tmux session to open in
    pub session: OsString,
    /// Name of the window
    pub name: OsString,
    /// Command to run
    pub command: OsString,
    /// Run on startup
    #[clap(short, long)]
    pub startup: bool,
}