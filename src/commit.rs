use regex::Regex;

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
    pub fn generate(&self) -> String {
        let scope = self.scope
            .as_ref()
            .map(|s| format!("({})", s))
            .unwrap_or_default();

        let breaking = if self.breaking_change { "!" } else { "" };

        let header = format!(
            "{}{}{}: {}",
            self.commit_type, scope, breaking, self.description
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

    pub fn validate(message: &str) -> Result<(), String> {
        let re = Regex::new(r"^(?P<type>[a-z]+)(\((?P<scope>[^()\r\n]*)\))?(?P<breaking>!)?: (?P<description>[^\r\n]+)").unwrap();

        if let Some(first_line) = message.lines().next() {
            if re.is_match(first_line) {
                Ok(())
            } else {
                Err(format!(
                    "Invalid commit message format. Must follow:\n<type>[optional scope]: <description>\n\nExample: feat(parser): add new parsing algorithm\n\nYour message: {}",
                    first_line
                ))
            }
        } else {
            Err("Commit message is empty".to_string())
        }
    }
}