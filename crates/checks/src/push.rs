use serde::{Deserialize, Serialize};

use crate::REDACTED;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushConfig {
    /// Secret in the URL the service calls to report in. The server
    /// generates one when it's left empty.
    #[serde(default)]
    pub token: String,
    /// Extra seconds after the interval before a missed heartbeat counts.
    #[serde(default)]
    pub grace_secs: u32,
}

impl PushConfig {
    /// Hide the token; anyone who has it can report for the monitor.
    pub fn redact(&mut self) {
        if !self.token.is_empty() {
            self.token = REDACTED.into();
        }
    }

    /// Keep `previous`'s token when this one is still the placeholder.
    pub fn keep_token_from(&mut self, previous: &PushConfig) {
        if self.token == REDACTED {
            self.token.clone_from(&previous.token);
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.token.len() < 16 {
            return Err("push token must be at least 16 characters".into());
        }
        if !self
            .token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err("push token may only contain letters, digits, - and _".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_redaction_round_trip() {
        let stored = PushConfig {
            token: "a1b2c3d4e5f6g7h8".into(),
            grace_secs: 30,
        };
        let mut shown = stored.clone();
        shown.redact();
        assert_eq!(shown.token, REDACTED);
        assert_eq!(shown.grace_secs, 30);

        let mut saved = shown.clone();
        saved.keep_token_from(&stored);
        assert_eq!(saved, stored);

        // A new token rotates it.
        let mut rotated = PushConfig {
            token: "new-token-0000000".into(),
            ..shown
        };
        rotated.keep_token_from(&stored);
        assert_eq!(rotated.token, "new-token-0000000");
    }

    #[test]
    fn validation() {
        let push = |token: &str| PushConfig {
            token: token.into(),
            grace_secs: 0,
        };
        assert!(push("a1B2-c3_d4e5f6g7h8").validate().is_ok());
        assert!(push("short").validate().is_err());
        assert!(push("has spaces in the token").validate().is_err());
        assert!(push("slash/in/token/value").validate().is_err());
    }
}
