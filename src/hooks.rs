use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const HOOK_SCRIPT: &str = r#"#!/bin/sh
# Convinci commit-msg hook
# Validates conventional commits format

# Pass commit message file content via stdin
cat "$1" | convinci validate -
"#;

pub fn install_hook() -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        anyhow::bail!("Not a Git repository");
    }

    let hooks_dir = git_dir.join("hooks");
    if !hooks_dir.exists() {
        fs::create_dir(&hooks_dir).context("Failed to create hooks directory")?;
    }

    let hook_path = hooks_dir.join("commit-msg");

    // Check if hook already exists
    if hook_path.exists() {
        let existing_content = fs::read_to_string(&hook_path).context("Failed to read existing hook")?;

        if existing_content.contains("convinci") {
            println!("Updating existing Convinci hook");
        } else {
            anyhow::bail!(
                "A commit-msg hook already exists. Please remove it and try again.\nPath: {}",
                hook_path.display()
            );
        }
    }

    // Write hook script
    let mut file = fs::File::create(&hook_path).context("Failed to create hook file")?;
    file.write_all(HOOK_SCRIPT.as_bytes())
        .context("Failed to write hook script")?;

    // Set executable permissions (Unix only)
    #[cfg(unix)]
    {
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_path, perms).context("Failed to set hook permissions")?;
    }

    println!("✅ Commit-msg hook installed at {}", hook_path.display());
    Ok(())
}

pub fn uninstall_hook() -> Result<()> {
    let hook_path = Path::new(".git/hooks/commit-msg");

    if !hook_path.exists() {
        println!("No Convinci hook found");
        return Ok(());
    }

    let content = fs::read_to_string(&hook_path).context("Failed to read hook file")?;
    if content.contains("convinci") {
        fs::remove_file(&hook_path).context("Failed to remove hook file")?;
        println!("✅ Commit-msg hook uninstalled");
    } else {
        println!("⚠️  Existing hook is not a Convinci hook. Leaving it untouched.");
    }

    Ok(())
}