use crate::{
    commit::ConventionalCommit,
    config::{COMMIT_SCOPES, COMMIT_TYPES},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InputField {
    Type,
    Scope,
    Description,
    Body,
    BreakingToggle,
    BreakingDescription,
    None,
}

#[derive(Debug)]
pub struct App {
    pub commit: ConventionalCommit,
    pub current_field: InputField,
    pub config: crate::config::AppConfig,
    pub should_quit: bool,
    pub list_state_type: ListState,
    pub list_state_scope: ListState,
    pub should_confirm: bool,
    pub single_field_mode: bool,
    pub show_help: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut list_state_type = ListState::default();
        list_state_type.select(Some(0));

        let mut list_state_scope = ListState::default();
        list_state_scope.select(Some(0));

        Self {
            commit: ConventionalCommit::default(),
            current_field: InputField::Type,
            config: crate::config::AppConfig::default(),
            should_quit: false,
            list_state_type,
            list_state_scope,
            should_confirm: false,
            single_field_mode: false,
            show_help: true,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn confirm_commit(&mut self) {
        self.should_confirm = true;
        self.should_quit = true;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Global Ctrl+C shortcut to exit
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        // Global Ctrl+Enter shortcut to confirm
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Enter {
            self.confirm_commit();
            return;
        }

        match self.current_field {
            InputField::Type => self.handle_type_selection(key),
            InputField::Scope => self.handle_scope_selection(key),
            InputField::Description => self.handle_description_input(key),
            InputField::Body => self.handle_body_input(key),
            InputField::BreakingToggle => self.handle_breaking_toggle(key),
            InputField::BreakingDescription => self.handle_breaking_description(key),
            InputField::None => self.handle_no_field(key),
        }
    }

    fn handle_no_field(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => self.focus_field(InputField::Type),
            KeyCode::BackTab => self.focus_field(InputField::BreakingDescription),
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Enter => self.confirm_commit(),
            _ => {}
        }
    }

    fn focus_field(&mut self, field: InputField) {
        self.current_field = field;
    }

    fn next_field(&mut self) {
        self.current_field = match self.current_field {
            InputField::Type => InputField::Scope,
            InputField::Scope => InputField::Description,
            InputField::Description => InputField::Body,
            InputField::Body => InputField::BreakingToggle,
            InputField::BreakingToggle => {
                if self.commit.breaking_change {
                    InputField::BreakingDescription
                } else {
                    InputField::Type
                }
            }
            InputField::BreakingDescription => InputField::Type,
            _ => InputField::Type,
        };
    }

    fn previous_field(&mut self) {
        self.current_field = match self.current_field {
            InputField::Type => InputField::BreakingDescription,
            InputField::Scope => InputField::Type,
            InputField::Description => InputField::Scope,
            InputField::Body => InputField::Description,
            InputField::BreakingToggle => InputField::Body,
            InputField::BreakingDescription => InputField::BreakingToggle,
            _ => InputField::Type,
        };
    }

    fn handle_type_selection(&mut self, key: KeyEvent) {
        let selected = self.list_state_type.selected().unwrap_or(0);
        let len = COMMIT_TYPES.len();

        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let next = (selected + 1) % len;
                self.list_state_type.select(Some(next));
                self.commit.commit_type = COMMIT_TYPES[next].to_string();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let prev = if selected == 0 { len - 1 } else { selected - 1 };
                self.list_state_type.select(Some(prev));
                self.commit.commit_type = COMMIT_TYPES[prev].to_string();
            }
            KeyCode::Tab => self.next_field(),
            KeyCode::BackTab => self.previous_field(),
            KeyCode::Esc => self.focus_field(InputField::None),
            KeyCode::Enter => self.confirm_commit(),
            // Selection by number (1-9)
            KeyCode::Char(c) if c.is_digit(10) => {
                if let Some(num) = c.to_digit(10) {
                    let idx = (num as usize) - 1;
                    if idx < len {
                        self.list_state_type.select(Some(idx));
                        self.commit.commit_type = COMMIT_TYPES[idx].to_string();
                        self.next_field();
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_scope_selection(&mut self, key: KeyEvent) {
        let len = COMMIT_SCOPES.len();
        let selected = self.list_state_scope.selected().unwrap_or(0);

        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let next = (selected + 1) % len;
                self.list_state_scope.select(Some(next));
                self.update_scope_value(next);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let prev = if selected == 0 { len - 1 } else { selected - 1 };
                self.list_state_scope.select(Some(prev));
                self.update_scope_value(prev);
            }
            KeyCode::Tab => self.next_field(),
            KeyCode::BackTab => self.previous_field(),
            KeyCode::Esc => self.focus_field(InputField::None),
            KeyCode::Enter => self.confirm_commit(),
            // Selection by number (1-9)
            KeyCode::Char(c) if c.is_digit(10) => {
                if let Some(num) = c.to_digit(10) {
                    let idx = (num as usize) - 1;
                    if idx < len {
                        self.list_state_scope.select(Some(idx));
                        self.update_scope_value(idx);
                        self.next_field();
                    }
                }
            }
            KeyCode::Backspace => {
                self.update_scope_value(0);
                self.list_state_scope.select(Some(0));
            }
            _ => {}
        }
    }

    fn update_scope_value(&mut self, index: usize) {
        if index == 0 {
            self.commit.scope = None;
        } else {
            self.commit.scope = Some(COMMIT_SCOPES[index].to_string());
        }
    }

    fn handle_description_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.commit.description.push(c);
            }
            KeyCode::Backspace => {
                self.commit.description.pop();
            }
            KeyCode::Tab => self.next_field(),
            KeyCode::BackTab => self.previous_field(),
            KeyCode::Esc => self.focus_field(InputField::None),
            KeyCode::Enter => self.confirm_commit(),
            _ => {}
        }
    }

    fn handle_body_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                let body = self.commit.body.get_or_insert(String::new());
                body.push(c);
            }
            KeyCode::Backspace => {
                if let Some(body) = &mut self.commit.body {
                    body.pop();
                }
            }
            KeyCode::Enter => {
                let body = self.commit.body.get_or_insert(String::new());
                body.push('\n');
            }
            KeyCode::Tab => self.next_field(),
            KeyCode::BackTab => self.previous_field(),
            KeyCode::Esc => self.focus_field(InputField::None),
            _ => {}
        }
    }

    fn handle_breaking_toggle(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(' ') => {
                self.commit.breaking_change = !self.commit.breaking_change;
            }
            KeyCode::Tab => self.next_field(),
            KeyCode::BackTab => self.previous_field(),
            KeyCode::Esc => self.focus_field(InputField::None),
            KeyCode::Enter => self.confirm_commit(),
            _ => {}
        }
    }

    fn handle_breaking_description(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.commit.breaking_change_description.push(c);
            }
            KeyCode::Backspace => {
                self.commit.breaking_change_description.pop();
            }
            KeyCode::Tab => self.next_field(),
            KeyCode::BackTab => self.previous_field(),
            KeyCode::Esc => self.focus_field(InputField::BreakingToggle),
            KeyCode::Enter => self.confirm_commit(),
            _ => {}
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        let size = f.area();
        self.single_field_mode = size.height < 25;

        if self.single_field_mode {
            self.render_single_field(f);
        } else {
            self.render_full_ui(f);
        }
    }

    fn render_single_field(&mut self, f: &mut Frame) {
        let area = f.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(5),    // Active field
                Constraint::Length(3), // Footer
            ])
            .split(area);

        self.render_header(f, chunks[0]);
        self.render_active_field(f, chunks[1]);
        self.render_footer(f, chunks[2]);
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        let title = match self.current_field {
            InputField::Type => "Commit Type",
            InputField::Scope => "Scope",
            InputField::Description => "Description",
            InputField::Body => "Commit Body",
            InputField::BreakingToggle => "Breaking Change",
            InputField::BreakingDescription => "Breaking Change Description",
            _ => "Conventional Commits Helper",
        };

        let block = Block::default()
            .title(format!(" Conventional Commits Helper - {} ", title))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        let progress = match self.current_field {
            InputField::Type => "Step 1/6",
            InputField::Scope => "Step 2/6",
            InputField::Description => "Step 3/6",
            InputField::Body => "Step 4/6",
            InputField::BreakingToggle => "Step 5/6",
            InputField::BreakingDescription => "Step 6/6",
            _ => "",
        };

        let header = Paragraph::new(progress)
            .block(block)
            .alignment(Alignment::Right);

        f.render_widget(header, area);
    }

    fn render_active_field(&mut self, f: &mut Frame, area: Rect) {
        match self.current_field {
            InputField::Type => self.render_type_field(f, area),
            InputField::Scope => self.render_scope_field(f, area),
            InputField::Description => self.render_description_field(f, area),
            InputField::Body => self.render_body_field(f, area),
            InputField::BreakingToggle => self.render_breaking_toggle(f, area),
            InputField::BreakingDescription => self.render_breaking_description(f, area),
            _ => {}
        }
    }

    fn render_full_ui(&mut self, f: &mut Frame) {
        let type_height = COMMIT_TYPES.len() as u16 + 2;
        let scope_height = COMMIT_SCOPES.len() as u16 + 2;

        // Reduced height for the commit body
        let body_height = 5;

        // Breaking change always takes 3 lines for the toggle + 3 for description
        let breaking_height = 6; // Now always fixed

        // Estimated total height
        let total_height = type_height + scope_height + 3 + body_height + breaking_height + 1;

        // If total height is greater than available, reduce more
        let available_height = f.area().height;
        let body_height = if total_height > available_height {
            // Calculate maximum available height for the body
            let max_body_height = available_height
                .saturating_sub(type_height + scope_height + 3 + breaking_height + 1);
            // Ensure at least 3 lines for the body
            max_body_height.max(3)
        } else {
            body_height
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(type_height),     // Type
                Constraint::Length(scope_height),    // Scope
                Constraint::Length(3),               // Description
                Constraint::Length(body_height),     // Body (reduced height)
                Constraint::Length(breaking_height), // Breaking change (always 6 lines)
                Constraint::Length(1),               // Footer
            ])
            .split(f.area());

        self.render_type_field(f, layout[0]);
        self.render_scope_field(f, layout[1]);
        self.render_description_field(f, layout[2]);
        self.render_body_field(f, layout[3]);
        self.render_breaking_field(f, layout[4]); // Now always renders complete
        self.render_footer(f, layout[5]);
    }

    fn render_type_field(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = COMMIT_TYPES
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let prefix = if self.list_state_type.selected() == Some(i) {
                    "▶ "
                } else {
                    "  "
                };
                ListItem::new(format!("{}{}. {}", prefix, i + 1, t))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Commit Type ")
                    .borders(Borders::ALL)
                    .border_style(if self.current_field == InputField::Type {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    }),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("▶");

        f.render_stateful_widget(list, area, &mut self.list_state_type);
    }

    fn render_scope_field(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = COMMIT_SCOPES
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let prefix = if self.list_state_scope.selected() == Some(i) {
                    "▶ "
                } else {
                    "  "
                };
                ListItem::new(format!("{}{}. {}", prefix, i + 1, s))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Scope (optional) ")
                    .borders(Borders::ALL)
                    .border_style(if self.current_field == InputField::Scope {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    }),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("▶");

        f.render_stateful_widget(list, area, &mut self.list_state_scope);
    }

    fn render_description_field(&self, f: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.commit.description.as_str())
            .style(Style::default())
            .block(
                Block::default()
                    .title(" Description ")
                    .borders(Borders::ALL)
                    .border_style(if self.current_field == InputField::Description {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    }),
            );

        f.render_widget(input, area);

        if self.current_field == InputField::Description {
            f.set_cursor(
                area.x + self.commit.description.len() as u16 + 1,
                area.y + 1,
            );
        }
    }

    fn render_body_field(&self, f: &mut Frame, area: Rect) {
        let body = self.commit.body.as_deref().unwrap_or("");

        let line_count = body.lines().count() as u16;
        let inner_height = area.height.saturating_sub(2);
        let offset_y = line_count.saturating_sub(inner_height);

        let input = Paragraph::new(body)
            .style(Style::default())
            .block(
                Block::default()
                    .title(" Body (optional) ")
                    .borders(Borders::ALL)
                    .border_style(if self.current_field == InputField::Body {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    }),
            )
            .scroll((offset_y, 0));

        f.render_widget(input, area);
    }

    fn render_breaking_toggle(&self, f: &mut Frame, area: Rect) {
        let checkbox = if self.commit.breaking_change {
            "[X]"
        } else {
            "[ ]"
        };
        let text = format!("{} Breaking Change (API incompatible)", checkbox);

        let block = Block::default()
            .title(" Significant Change ")
            .borders(Borders::ALL)
            .border_style(if self.current_field == InputField::BreakingToggle {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            });

        let paragraph = Paragraph::new(text)
            .block(block)
            .style(if self.commit.breaking_change {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            });

        f.render_widget(paragraph, area);
    }

    fn render_breaking_description(&self, f: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.commit.breaking_change_description.as_str())
            .style(Style::default())
            .block(
                Block::default()
                    .title(" Breaking Change Description ")
                    .borders(Borders::ALL)
                    .border_style(if self.current_field == InputField::BreakingDescription {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default()
                    }),
            );

        f.render_widget(input, area);

        if self.current_field == InputField::BreakingDescription {
            f.set_cursor(
                area.x + self.commit.breaking_change_description.len() as u16 + 1,
                area.y + 1,
            );
        }
    }

    fn render_breaking_field(&self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Toggle (always visible)
                Constraint::Length(3), // Description (always visible)
            ])
            .split(area);

        self.render_breaking_toggle(f, layout[0]);

        // Now always render the description, even when it's not active
        if self.commit.breaking_change {
            self.render_breaking_description(f, layout[1]);
        } else {
            // When not active, show empty field
            let input = Paragraph::new("")
                .style(Style::default().fg(Color::DarkGray))
                .block(
                    Block::default()
                        .title(" Breaking Change Description (disabled) ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray)),
                );

            f.render_widget(input, layout[1]);
        }
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let keys_hint = if self.show_help {
            match self.current_field {
                InputField::None => {
                    "x: Settings  Tab: Focus field  Esc: Exit  Ctrl+C: Exit  Ctrl+Enter: Confirm"
                }
                InputField::Type | InputField::Scope => {
                    "↑/↓/hjkl: Navigate  1-9: Direct selection  Tab: Next  Shift+Tab: Previous  Enter: Confirm"
                }
                InputField::Description | InputField::Body => {
                    "←→/hl: Fields  Tab: Navigate  Enter: Confirm  Esc: Defocus"
                }
                InputField::BreakingToggle => {
                    "Space: Toggle  Tab: Next  Shift+Tab: Previous  Enter: Confirm"
                }
                InputField::BreakingDescription => {
                    "Type description  Enter: Confirm  Esc: Back"
                }
            }
        } else {
            "Press 'h' for help"
        };

        // Add mode indicator
        let mode_indicator = if self.config.dev_mode {
            " [DEV MODE] "
        } else {
            " [GIT MODE] "
        };

        let footer_text = format!("{}{}", mode_indicator, keys_hint);

        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);

        f.render_widget(footer, area);
    }
}