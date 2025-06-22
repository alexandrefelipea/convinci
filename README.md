# Convinci - Conventional Commits Helper

[![GitHub release](https://img.shields.io/github/release/alexandrefelipea/convinci.svg)](https://github.com/alexandrefelipea/convinci/releases)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Convinci is a fast and simple terminal application that helps you create conventional commits with an interactive TUI (Text-based User Interface). It guides you step-by-step to create commits that follow the [Conventional Commits](https://www.conventionalcommits.org) specification, making your commit history more readable and standardized.

## Features

- 🚀 **Fast and intuitive** TUI interface

- 📝 Step-by-step commit message creation

- 🔄 Support for all conventional commit types (feat, fix, chore, docs, etc.)
- ⚠️ Breaking change indicator and description
- 📦 Pre-built binaries for Linux, macOS, and Windows

## Installation

### Linux/macOS
```bash
curl -sSL https://raw.githubusercontent.com/alexandrefelipea/convinci/master/install.sh | bash
```
Or with wget:

```bash
wget -qO- https://raw.githubusercontent.com/seu-usuario-github/convinci/master/install.sh | bash
```
### Windows
1. Open PowerShell as an administrator
2. Run:
```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/alexandrefelipea/convinci/master/install.ps1'))
```

### Alternative method for all platforms (with Rust installed):
```bash
cargo install convinci
```

## Update
### Linux/macOS
```bash
curl -sSL https://raw.githubusercontent.com/alexandrefelipea/convinci/master/install.sh | bash
```
Or with wget:

```bash
wget -qO- https://raw.githubusercontent.com/seu-usuario-github/convinci/master/install.sh | bash
```

### Windows
Run the installation script again.


### Via cargo
```bash
cargo install convinci --force
```

## Uninstallation
### Linux/macOS
```bash
sudo rm /usr/local/bin/convinci
```

### Windows
1. Delete the file `convinci.exe` from the installation directory (usually `%USERPROFILE%\bin`)
2. Remove the directory from PATH if desired

## Usage

Run Convinci in your Git repository:

```bash

convinci

```

Follow the interactive prompts to create your commit. The interface is designed to be intuitive and fast.

### Keybindings

- **Tab**: Move to next field

- **Shift+Tab**: Move to previous field

- **Ctrl+Enter**: Confirm and generate commit

- **Ctrl+C**: Exit without committing

- **Arrow keys / HJKL**: Navigate lists

- **1-9**: Quick selection from lists

## Contributing

We welcome contributions! Here's how to set up your development environment:

### Prerequisites

- Rust (latest stable version)

- Git

- Basic terminal tools (make, curl, etc.)

### Setup

1. Fork the repository

2. Clone your fork:

```bash

git clone https://github.com/alexandrefelipea/convinci.git

cd convinci

```

3. Build the project:

```bash

cargo build

```

4. Run in development mode:

```bash

cargo run

```

### Contribution Guidelines

- Follow the existing code style

- Update documentation when needed

- Use descriptive commit messages (Convinci can help with that!)

- Open a pull request against the `master` branch

### Reporting Issues

Please use the [GitHub issue tracker](https://github.com/alexandrefelipea/convinci/issues) to report bugs or request features.

## License

Convinci is open-source software licensed under the [MIT license](LICENSE).
