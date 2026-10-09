use serde::{Deserialize, Serialize};

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
