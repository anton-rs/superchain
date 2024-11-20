//! Validation mode for payload validation.

/// The payload validation mode.
///
/// Every newly derived payload needs to be validated against a local
/// execution of all transactions included inside it. This can be done
/// in two ways:
///
/// - Trusted: rely on a trusted synced L2 execution client. Validation happens by fetching the same
///   block and comparing the results.
/// - Engine API: use the authenticated engine API of an L2 execution client. Validation happens by
///   sending the `new_payload` to the API and expecting a VALID response. This method can also be
///   used to verify unsafe payloads from the sequencer.
#[derive(Debug, Clone)]
pub enum ValidationMode {
    /// Use a trusted synced L2 execution client.
    Trusted,
    /// Use the authenticated engine API of an L2 execution client.
    EngineApi,
}

impl std::str::FromStr for ValidationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trusted" => Ok(ValidationMode::Trusted),
            "engine-api" => Ok(ValidationMode::EngineApi),
            _ => Err(format!("Invalid validation mode: {}", s)),
        }
    }
}

impl std::fmt::Display for ValidationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationMode::Trusted => write!(f, "trusted"),
            ValidationMode::EngineApi => write!(f, "engine-api"),
        }
    }
}
