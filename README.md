# Convinci 🚀 - Conventional Commits Helper

[![Crate Version](https://img.shields.io/crates/v/convinci.svg)](https://crates.io/crates/convinci)
[![GitHub Release](https://img.shields.io/github/v/release/alexandrefelipea/convinci?include_prereleases)](https://github.com/alexandrefelipea/convinci/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-orange.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/alexandrefelipea/convinci/release.yml)](https://github.com/alexandrefelipea/convinci/actions)

**Convinci** is a fast, intuitive terminal application that guides you through creating perfect [Conventional Commits](https://www.conventionalcommits.org) with an interactive TUI. Standardize your commit history and make it more readable in minutes!


## ✨ Features

- 🚀 **Fast** TUI interface
- 📝 **Step-by-step guidance** for perfect commits
- 🔄 Support for all standard commit types (`feat`, `fix`, `chore`, `docs`, etc.)
- ⚠️ Breaking change indicator and description
- 🌈 Optional emoji support for visual commit messages
- 📦 Pre-built binaries for all major platforms
- ⌨️ Vim-style navigation (`hjkl`) and keyboard shortcuts

## 📦 Installation
### Linux/macOS

Install with curl:
```bash
curl -sSL https://raw.githubusercontent.com/alexandrefelipea/convinci/master/install.sh | bash
```
Or with wget:
```bash
wget -qO- https://raw.githubusercontent.com/alexandrefelipea/convinci/master/install.sh | bash
```
### Windows (PowerShell)

Run as Administrator:
```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "iex (Invoke-RestMethod -Uri 'https://raw.githubusercontent.com/alexandrefelipea/convinci/master/install.ps1')"
```
### Via Cargo (Rust developers)
```bash
cargo install convinci
```
## 🔄 Updating
Update to the latest version using the same installation methods for Linux/macOS and Windows.

Updating Via Cargo:
```bash 
cargo install convinci --force
```
## 💻 Usage
Navigate to your Git repository and run:
```bash
convinci
```
### Command Options
| Option          | Description                          | Example                     |
|-----------------|--------------------------------------|-----------------------------|
| `-e`, `--emoji` | Enable emojis in commit messages     | `convinci --emoji`          |
| `-d`, `--demo`  | Demo mode (no actual commit)         | `convinci --demo`           |
| `-v`, `--version` | Show version information            | `convinci --version`        |
| `-h`, `--help`   | Show help message              | `convinci --help`           |
### Advanced Usage
```bash
# Create commit with emojis
convinci --emoji
# Preview commit without executing
convinci --demo
# Combine options
convinci -e -d  # Demo mode with emojis
```
### Keybindings
| Keys               | Action                          |
|--------------------|---------------------------------|
| `Tab`/`Shift+Tab` | Navigate between fields         |
| `Ctrl+Enter`       | Confirm and generate commit     |
| `Ctrl+C`           | Exit without committing         |
| `↑↓`/`jk`        | Navigate lists and options      |
| `0-9`              | Quick select from numbered list |
## ❌ Uninstallation
### Linux/macOS
```bash
sudo rm $(which convinci)
```
### Windows
1. Delete `convinci.exe` from your bin directory (`%USERPROFILE%\bin`)
2. Remove the directory from your PATH if needed
## 🤝 Contributing
We welcome contributions! Here's how to get started:
### Prerequisites
- Rust (latest stable)
- Git
### Setup & Development
```bash
# 1. Fork and clone the repository
git clone https://github.com/your-username/convinci.git
cd convinci
# 2. Build the project
cargo build
# 3. Run in development mode
cargo run -- --demo  # Test with demo mode
```
### Contribution Guidelines
- Follow existing code style and patterns
- Update documentation when changing features
- Use descriptive commit messages (Convinci can help!)
- Open pull requests against the `master` branch
### Reporting Issues
Found a bug? Have a feature request? Please [open an issue](https://github.com/alexandrefelipea/convinci/issues).
## 📜 License
Convinci is open-source software licensed under the [MIT License](LICENSE).
