use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventParseError {
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Invalid event format")]
    InvalidEventFormat,
    #[error("Unknown event type: {0}")]
    UnknownEventType(String),
}

pub struct EventParser;

impl EventParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_logs(&self, logs: &[String]) -> Result<Vec<ParsedEvent>, EventParseError> {
        let mut events = Vec::new();
        for log in logs {
            if log.starts_with("Program log: ") {
                if let Some(event) = self.parse_program_log(log) {
                    events.push(event?);
                }
            }
        }
        Ok(events)
    }

    fn parse_program_log(&self, log: &str) -> Option<Result<ParsedEvent, EventParseError>> {
        let content = log.trim_start_matches("Program log: ");
        if content.starts_with('{') && content.ends_with('}') {
            Some(self.parse_json_event(content))
        } else {
            None
        }
    }

    fn parse_json_event(&self, json_str: &str) -> Result<ParsedEvent, EventParseError> {
        let json: Value = serde_json::from_str(json_str)?;

        let event_type = json["type"]
            .as_str()
            .ok_or(EventParseError::InvalidEventFormat)?;

        match event_type {
            "cypher_transfer" => self.parse_cypher_transfer_event(&json),
            "cypher_mint" => self.parse_cypher_mint_event(&json),
            "cypher_burn" => self.parse_cypher_burn_event(&json),
            _ => Err(EventParseError::UnknownEventType(event_type.to_string())),
        }
    }

    fn parse_cypher_transfer_event(&self, json: &Value) -> Result<ParsedEvent, EventParseError> {
        Ok(ParsedEvent::CypherTransfer {
            from: json["from"]
                .as_str()
                .ok_or(EventParseError::InvalidEventFormat)?
                .to_string(),
            to: json["to"]
                .as_str()
                .ok_or(EventParseError::InvalidEventFormat)?
                .to_string(),
            amount: json["amount"]
                .as_u64()
                .ok_or(EventParseError::InvalidEventFormat)?,
        })
    }

    fn parse_cypher_mint_event(&self, json: &Value) -> Result<ParsedEvent, EventParseError> {
        Ok(ParsedEvent::CypherMint {
            to: json["to"]
                .as_str()
                .ok_or(EventParseError::InvalidEventFormat)?
                .to_string(),
            amount: json["amount"]
                .as_u64()
                .ok_or(EventParseError::InvalidEventFormat)?,
        })
    }

    fn parse_cypher_burn_event(&self, json: &Value) -> Result<ParsedEvent, EventParseError> {
        Ok(ParsedEvent::CypherBurn {
            from: json["from"]
                .as_str()
                .ok_or(EventParseError::InvalidEventFormat)?
                .to_string(),
            amount: json["amount"]
                .as_u64()
                .ok_or(EventParseError::InvalidEventFormat)?,
        })
    }
}

#[derive(Debug)]
pub enum ParsedEvent {
    CypherTransfer {
        from: String,
        to: String,
        amount: u64,
    },
    CypherMint {
        to: String,
        amount: u64,
    },
    CypherBurn {
        from: String,
        amount: u64,
    },
}
