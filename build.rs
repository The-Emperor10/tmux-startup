use clap::IntoApp;
use clap_generate::generators::Zsh;
use clap_generate::{generate_to, generators::Bash};
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let mut app = Cli::into_app();

    generate_to(Bash, &mut app, "tmux-startup", ".")?;

    generate_to(Zsh, &mut app, "tmux-startup", ".")?;

    Ok(())
}
