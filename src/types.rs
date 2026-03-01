//! Local types replacing panduza-interfaces.
//!
//! Defines power supply types previously provided by the MQTT-based
//! panduza-interfaces crate.

/// Power output state: On or Off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnOffValue {
    /// Output enabled
    On,
    /// Output disabled
    Off,
}

impl std::fmt::Display for OnOffValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OnOffValue::On => write!(f, "on"),
            OnOffValue::Off => write!(f, "off"),
        }
    }
}

impl OnOffValue {
    /// Parse from a string ("on"/"off"), case-insensitive.
    #[allow(dead_code)]
    pub fn from_str_lossy(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "on" => Some(OnOffValue::On),
            "off" => Some(OnOffValue::Off),
            _ => None,
        }
    }
}
