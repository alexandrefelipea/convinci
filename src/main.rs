mod commit;
mod config;
mod hooks;
mod tui;

use crate::config::AppConfig;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{io, panic, process::Command};
use std::io::Read;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Convinci - A fast terminal helper for creating conventional commits",
    long_about = r#"
Convinci is a fast and simple terminal application that helps you create conventional commits with an interactive TUI.

It guides you step-by-step to create commits that follow the Conventional Commits specification.

Key features:
- Interactive TUI interface
- Step-by-step commit creation
- Support for all conventional commit types
- Breaking change indicator and description
- Demo mode to preview commits without committing
- Git hooks integration for commit validation

Usage:
  convinci [OPTIONS] [COMMAND]

Options:
  -d, --demo     Run in demo mode (no actual commit, interactive mode only)

Commands:
  hooks          Manage Git hooks for commit validation
  validate       Validate a commit message
  help           Print this message or the help of the given subcommand(s)

Examples:
  convinci              # Run interactive mode (default)
  convinci --demo       # Run in demo mode (only prints the commit)
  convinci hooks install # Install commit-msg hook
  convinci validate "feat: add new feature" # Validate a commit message
  convinci hooks uninstall # Uninstall commit-msg hook
"#
)]
struct Cli {
    #[arg(short, long)]
    demo: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Hooks {
        #[command(subcommand)]
        command: HooksCommand,
    },

    Validate {
        message: String,
    },
}

#[derive(Subcommand, Debug)]
enum HooksCommand {
    Install,
    Uninstall,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Hooks { command }) => match command {
            HooksCommand::Install => hooks::install_hook(),
            HooksCommand::Uninstall => hooks::uninstall_hook(),
        },
        Some(Commands::Validate { message }) => {
            let message = if message == "-" {
                // Read from stdin
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                buffer
            } else {
                message
            };

            validate_commit_message(&message)?;
            println!("✅ Commit message is valid!");
            Ok(())
        },
        None => run_interactive(cli.demo),
    }
}

fn run_interactive(dev_mode: bool) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Panic hook
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        reset_terminal().expect("Failed to reset the terminal during panic");
        panic_hook(panic);
    }));

    // Create app with config from arguments
    let mut app = tui::App::new();
    app.config = AppConfig {
        dev_mode,
    };

    run_app(&mut terminal, &mut app)?;

    // Final cleanup
    reset_terminal()?;
    if app.should_confirm {
        let commit_message = app.commit.generate();

        if dev_mode {
            println!("Generated commit message:\n\n{}", commit_message);
        } else {
            perform_git_commit(&commit_message)?;
        }
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut tui::App) -> Result<()> {
    while !app.should_quit {
        terminal.draw(|f| app.render(f))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key(key);

                if key.code == KeyCode::Char('q') && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                    app.should_quit = true;
                }
            }
        }
    }
    Ok(())
}

fn validate_commit_message(message: &str) -> Result<()> {
    commit::ConventionalCommit::validate(message)
        .map_err(|e| anyhow::anyhow!(e))
}

fn perform_git_commit(message: &str) -> Result<()> {
    // Check if we are in a Git repository
    let repo_check = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .context("Failed to check Git repository")?;

    if !repo_check.status.success() {
        anyhow::bail!("Not inside a Git repository");
    }

    // Execute the commit
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()
        .context("Failed to execute git commit")?;

    if status.success() {
        println!("✅ Commit successful!");
    } else {
        anyhow::bail!("Error executing git commit");
    }

    Ok(())
}

fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}