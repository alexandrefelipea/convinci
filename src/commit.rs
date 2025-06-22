use crate::config::AppConfig;

#[derive(Debug)]
pub struct ConventionalCommit {
    pub commit_type: String,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub breaking_change: bool,
    pub breaking_change_description: String,
}

impl Default for ConventionalCommit {
    fn default() -> Self {
        Self {
            commit_type: "feat".to_string(), // Initialize with a default value
            scope: None,
            description: String::new(),
            body: None,
            breaking_change: false,
            breaking_change_description: "".to_string(),
        }
    }
}

impl ConventionalCommit {
    pub fn generate(&self, config: &AppConfig) -> String {
        let emoji = if config.use_emoji {
            match self.commit_type.as_str() {
                "feat" => "✨ ",
                "fix" => "🐛 ",
                "docs" => "📚 ",
                "style" => "🎨 ",
                "refactor" => "♻️ ",
                "perf" => "⚡ ",
                "test" => "✅ ",
                "build" => "📦 ",
                "ci" => "👷 ",
                "chore" => "🔧 ",
                _ => "",
            }
        } else {
            ""
        };

        let scope = self.scope
            .as_ref()
            .map(|s| format!("({})", s))
            .unwrap_or_default();

        let breaking = if self.breaking_change { "!" } else { "" };

        let header = format!(
            "{}{}{}{}: {}",
            emoji, self.commit_type, scope, breaking, self.description
        );

        let body = self.body
            .as_ref()
            .map(|b| format!("\n\n{}", b))
            .unwrap_or_default();

        let footer = if self.breaking_change {
            let breaking_desc = if !self.breaking_change_description.is_empty() {
                format!("\n\nBREAKING CHANGE: {}", self.breaking_change_description)
            } else {
                "".to_string()
            };
            breaking_desc
        } else {
            "".to_string()
        };

        format!("{}{}{}", header, body, footer)
    }
}