use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Msg(String),
    #[error("{code}: {message}")]
    Api { status: u16, code: String, message: String, retry_after: Option<u64> },
    #[error("network: {0}")]
    Network(String),
    #[error("could not save the refresh token: {0}")]
    TokenSave(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Error {
    pub fn msg(text: impl Into<String>) -> Self {
        Self::Msg(text.into())
    }

    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn code(&self) -> Option<&str> {
        match self {
            Self::Api { code, .. } => Some(code),
            _ => None,
        }
    }

    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Api { retry_after: Some(secs), .. } => Some(Duration::from_secs(*secs)),
            _ => None,
        }
    }

    pub fn is_unauthorized(&self) -> bool {
        self.status() == Some(401) || self.code() == Some("invalid_grant") || self.code() == Some("unauthorized")
    }

    pub fn is_device_mismatch(&self) -> bool {
        self.code() == Some("device_mismatch")
    }

    pub fn is_rate_limited(&self) -> bool {
        self.status() == Some(429) || self.code() == Some("rate_limited")
    }

    pub fn is_catalog_unavailable(&self) -> bool {
        self.code() == Some("catalog_unavailable")
    }

    pub fn is_connect(&self) -> bool {
        matches!(self, Self::Network(text) if text.starts_with("connect:"))
    }

    pub fn is_network(&self) -> bool {
        matches!(self, Self::Network(_))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// How long to wait after HTTP 429 or catalog 503. Clamped so a bad header cannot spin or stall forever.
pub fn backoff_secs(err: &Error, default_secs: u64) -> u64 {
    let raw = match err {
        Error::Api { retry_after: Some(secs), .. } => *secs,
        _ => default_secs,
    };
    raw.clamp(1, 15 * 60)
}
