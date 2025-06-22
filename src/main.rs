mod commit;
mod config;
mod tui;

use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{io, panic, process::Command};
use tui::App;

fn main() -> Result<()> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Panic hook to restore the terminal
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        reset_terminal().expect("Failed to reset the terminal during panic");
        panic_hook(panic);
    }));

    // Create and run the application
    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    // Final cleanup
    reset_terminal()?;
    if app.should_confirm {
        let commit_message = app.commit.generate(&app.config);

        if app.config.dev_mode {
            println!("Generated commit (developer mode):\n\n{}", commit_message);
        } else {
            perform_git_commit(&commit_message)?;
        }
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    while !app.should_quit {
        terminal.draw(|f| app.render(f))?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key);

            // Global shortcut to exit
            if key.code == KeyCode::Char('q') && key.modifiers.contains(event::KeyModifiers::CONTROL) {
                app.should_quit = true;
            }
        }
    }
    Ok(())
}

// Function to perform the actual commit
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