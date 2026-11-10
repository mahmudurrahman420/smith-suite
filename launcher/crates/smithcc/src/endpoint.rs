use serde::{Deserialize, Serialize};

/// Which backend the launcher talks to. Production is the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Prod,
    Staging,
    Mock,
}

pub const PROD_API_BASE: &str = "https://slmptech.com/api/v1";
pub const STAGING_API_BASE: &str = "https://smith-cc-api-staging.mahmudur76.workers.dev/api/v1";
pub const STAGING_HOST: &str = "smith-cc-api-staging.mahmudur76.workers.dev";

impl Environment {
    pub fn parse(text: &str) -> Option<Self> {
        match text.trim().to_ascii_lowercase().as_str() {
            "prod" | "production" => Some(Self::Prod),
            "staging" | "stage" => Some(Self::Staging),
            "mock" | "dev" => Some(Self::Mock),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Prod => "prod",
            Self::Staging => "staging",
            Self::Mock => "mock",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Prod => "Production",
            Self::Staging => "Staging",
            Self::Mock => "Mock",
        }
    }

    pub fn default_base(self) -> &'static str {
        match self {
            Self::Prod => PROD_API_BASE,
            Self::Staging => STAGING_API_BASE,
            Self::Mock => "http://127.0.0.1",
        }
    }
}

/// The staging test-user sign-in (`POST /dev/sessions/{id}/approve`) is allowed only for the
/// real staging origin. Production, mock, and any other host are refused before a request is sent.
pub fn dev_approve_allowed(base: &str) -> bool {
    let Ok(url) = reqwest::Url::parse(base.trim()) else {
        return false;
    };
    if url.scheme() != "https" || !url.username().is_empty() || url.password().is_some() {
        return false;
    }
    if url.host_str() != Some(STAGING_HOST) || url.port().is_some() {
        return false;
    }
    if url.query().is_some() || url.fragment().is_some() {
        return false;
    }
    url.path().trim_end_matches('/') == "/api/v1"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dev_approve_is_staging_host_only() {
        assert!(dev_approve_allowed(STAGING_API_BASE));
        assert!(dev_approve_allowed("https://smith-cc-api-staging.mahmudur76.workers.dev/api/v1/"));
        assert!(!dev_approve_allowed(PROD_API_BASE));
        assert!(!dev_approve_allowed("http://smith-cc-api-staging.mahmudur76.workers.dev/api/v1"));
        assert!(!dev_approve_allowed("https://smith-cc-api-staging.mahmudur76.workers.dev.evil.com/api/v1"));
        assert!(!dev_approve_allowed("https://evil.com/api/v1"));
        assert!(!dev_approve_allowed("https://user@smith-cc-api-staging.mahmudur76.workers.dev/api/v1"));
        assert!(!dev_approve_allowed("https://smith-cc-api-staging.mahmudur76.workers.dev/api/v1/dev/sessions/x/approve"));
        assert!(!dev_approve_allowed("http://127.0.0.1:9/api/v1"));
    }
}
