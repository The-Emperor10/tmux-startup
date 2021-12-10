use std::{
    ffi::{OsStr},
    fs,
    io::{Read, Write},
    path::PathBuf,
    process::Stdio,
};

use clap::{Parser};
mod cli;
use cli::*;

fn main() {
    let args = Cli::parse();
    // Config folder check
    let config_dir_path: PathBuf = {
        if let Ok(path) = std::env::var("TMUX_STARTUP_HOME") {
            path.into()
        } else if let Ok(path) = std::env::var("XDG_CONFIG_HOME") {
            Into::<PathBuf>::into(path).join("tmux_startup")
        } else if let Ok(path) = std::env::var("HOME") {
            Into::<PathBuf>::into(path).join(".config/tmux_startup")
        } else {
            println!("Please set $HOME, $XDG_CONFIG_HOME, or $TMUX_STARTUP_HOME");
            std::process::exit(2);
        }
    };

    let config_file_path = config_dir_path.join("tmux_startup.json");
    fs::create_dir_all(&config_dir_path).unwrap();

    let mut config: Vec<Command> = {
        let mut config_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&config_file_path)
            .unwrap();
        let mut buf = Vec::new();
        config_file.read_to_end(&mut buf).unwrap();
        serde_json::from_slice(&buf).unwrap_or_default()
    };

    let mut modified_config = false;

    match args.command {
        Commands::Startup => config.iter().filter(|p| p.startup).for_each(create_window),
        Commands::List => config.iter().for_each(|p| println!("{:?}", p)),
        Commands::Add {
            command: program,
            overwrite,
        } => {
            let existing = config.iter_mut().find(|p| p.name == program.name);
            if existing.is_none() {
                config.push(program);
                modified_config = true;
            } else if overwrite {
                if let Some(existing) = existing {
                    *existing = program;
                    modified_config = true;
                }
            } else {
                println!(
                    "Command by the name of {:?} already exists. Use --overwrite to overwrite it.",
                    program.name
                );
                std::process::exit(1);
            }
        }
        Commands::Remove { name } => {
            config.retain(|p| p.name != name);
            modified_config = true;
        }
        Commands::Start { command: program } => config
            .iter()
            .filter(|p| p.name == program)
            .for_each(create_window),
        Commands::Stop { all, force, name } => {
            if all {
                for i in &config {
                    if force {
                        close_window(&i.name);
                    } else {
                        get_window_process_pids(&i.name)
                            .iter()
                            .for_each(|pid| unsafe {
                                libc::kill(*pid, libc::SIGTERM);
                            });
                    }
                }
            } else if let Some(name) = name {
                if force {
                    close_window(name);
                } else {
                    get_window_process_pids(name).iter().for_each(|pid| unsafe {
                        libc::kill(*pid, libc::SIGTERM);
                    })
                }
            }
        }
        Commands::CheckRunning { all, name } => {
            if all {
                for i in &config {
                    if !get_window_process_pids(&i.name).is_empty() {
                        println!("{}", &i.name.to_string_lossy());
                    }
                }
            } else if let Some(name) = name {
                if !get_window_process_pids(&name).is_empty() {
                    println!("{}", &name.to_string_lossy());
                }
            }
        }
    };

    if modified_config {
        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&config_file_path)
            .unwrap();
        let config_string = serde_json::to_string_pretty(&config).unwrap();
        config_file.write_all(config_string.as_bytes()).unwrap();
    }
}

fn create_window(program: &Command) {
    if !get_window_process_pids(&program.name).is_empty() {
        return;
    }
    if !is_session_running(&program.session) {
        std::process::Command::new("tmux")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("new-session")
            .arg("-ds")
            .arg(&program.session)
            .arg(&program.command)
            .arg(";")
            .arg("rename-window")
            .arg(&program.name)
            .output()
            .unwrap();
    } else {
        std::process::Command::new("tmux")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("new-window")
            .arg("-dt")
            .arg(&program.session)
            .arg("-n")
            .arg(&program.name)
            .arg(&program.command)
            .spawn()
            .unwrap();
    }
}

fn close_window<T>(name: T)
where
    T: AsRef<OsStr>,
{
    std::process::Command::new("tmux")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("kill-window")
        .arg("-t")
        .arg(name)
        .spawn()
        .unwrap();
}

fn get_window_process_pids<T>(name: T) -> Vec<i32>
where
    T: AsRef<OsStr>,
{
    String::from_utf8(
        std::process::Command::new("tmux")
            .arg("list-panes")
            .arg("-a")
            .arg("-F")
            .arg("#{pane_pid} #{window_name}")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .split('\n')
    .filter_map(|s| {
        let (pid_string, win_name) = s.split_once(' ')?;
        if name.as_ref() == win_name {
            Some(pid_string.parse().ok()?)
        } else {
            None
        }
    })
    .collect()
}

fn is_session_running<T>(name: T) -> bool
where
    T: AsRef<OsStr>,
{
    String::from_utf8(
        std::process::Command::new("tmux")
            .arg("list-sessions")
            .arg("-F")
            .arg("#{session_name}")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .split('\n')
    .any(|s| s == name.as_ref())
}
