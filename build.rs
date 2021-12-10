use clap::IntoApp;
use clap_generate::generators::Zsh;
use clap_generate::{generate_to, generators::Bash};
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
	let mut app = Cli::into_app();

    generate_to(
        Bash,
        &mut app, // We need to specify what generator to use
        "tmux-startup",  // We need to specify the bin name manually
        ".",   // We need to specify where to write to
    )?;

    generate_to(
        Zsh,
        &mut app, // We need to specify what generator to use
        "tmux-startup",  // We need to specify the bin name manually
        ".",   // We need to specify where to write to
    )?;

    Ok(())
}