pub struct EventParser;

impl EventParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_logs(&self, logs: &[String]) -> Result<Vec<ParsedEvent>, Box<dyn std::error::Error>> {
        let mut events = Vec::new();
        for log in logs {
            if log.starts_with("Program log: ") {
                if let Some(event) = self.parse_program_log(log) {
                    events.push(event);
                }
            }
        }
        Ok(events)
    }

    fn parse_program_log(&self, log: &str) -> Option<ParsedEvent> {
        let content = log.trim_start_matches("Program log: ");
        if content.starts_with('{') && content.ends_with('}') {
            if let Ok(json) = serde_json::from_str(content) {
                return Some(ParsedEvent::Json(json));
            }
        }
        Some(ParsedEvent::Plain(content.to_string()))
    }
}

pub enum ParsedEvent {
    Json(serde_json::Value),
    Plain(String),
}