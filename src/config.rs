pub const COMMIT_TYPES: [&str; 10] = [
    "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
];

pub const COMMIT_SCOPES: [&str; 9] = [
    "<None>", "ui", "api", "database", "auth", "config", "logging", "network", "storage",
];

#[derive(Debug, Clone, Copy)]
pub struct AppConfig {
    pub dev_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            dev_mode: false,
        }
    }
}
