//! Refresh-token handling.
//!
//! A refresh token rotates on every use. Reusing the previous one revokes every token for the
//! device, so the new token is written to the store before it is used, and a failed refresh is
//! never automatically retried with the token that was just sent.

use std::time::{Duration, Instant};

use crate::api::{ApiClient, Issued};
use crate::error::{Error, Result};
use crate::store::TokenStore;

pub struct Vault {
    store: Box<dyn TokenStore>,
    refresh: Option<String>,
    access: Option<String>,
    obtained: Option<Instant>,
    expires_in: u64,
}

impl Vault {
    pub fn new(store: Box<dyn TokenStore>) -> Self {
        Self { store, refresh: None, access: None, obtained: None, expires_in: 0 }
    }

    pub fn load(&mut self) -> Result<()> {
        self.refresh = self.store.load()?;
        self.access = None;
        self.obtained = None;
        Ok(())
    }

    pub fn has_refresh(&self) -> bool {
        self.refresh.as_ref().is_some_and(|t| !t.is_empty())
    }

    pub fn accept_issued(&mut self, issued: &Issued) -> Result<()> {
        self.persist_new(&issued.refresh_token)?;
        self.access = Some(issued.access_token.clone());
        self.expires_in = issued.expires_in;
        self.obtained = Some(Instant::now());
        Ok(())
    }

    /// One refresh attempt. The token that was sent is forgotten unless the connection never
    /// reached the server (`connect:`), in which case it is restored for a later manual try.
    pub fn rotate(&mut self, api: &ApiClient, device_id: &str) -> Result<()> {
        let Some(old) = self.refresh.take() else {
            return Err(Error::msg("sign in to continue"));
        };
        match api.refresh(&old, device_id) {
            Ok(issued) => {
                self.access = Some(issued.access_token);
                self.expires_in = issued.expires_in;
                self.obtained = Some(Instant::now());
                // persist_new drops the previous token from the store. A failed write clears
                // the old token so the next launch cannot reuse it and revoke this one.
                self.persist_new(&issued.refresh_token)?;
                Ok(())
            }
            Err(err) if err.is_connect() => {
                self.refresh = Some(old);
                Err(err)
            }
            Err(err) => {
                // The request reached the server (or timed out after it might have).
                // Forget the token that was sent so nothing retries it.
                let _ = self.store.clear();
                self.access = None;
                Err(err)
            }
        }
    }

    pub fn access(&mut self, api: &ApiClient, device_id: &str) -> Result<String> {
        if self.access_is_fresh() {
            return self.access.clone().ok_or_else(|| Error::msg("sign in to continue"));
        }
        self.rotate(api, device_id)?;
        self.access.clone().ok_or_else(|| Error::msg("sign in to continue"))
    }

    pub fn clear(&mut self) -> Result<()> {
        self.refresh = None;
        self.access = None;
        self.obtained = None;
        self.store.clear()
    }

    fn persist_new(&mut self, refresh: &str) -> Result<()> {
        self.refresh = Some(refresh.to_string());
        if self.store.save(refresh).is_ok() {
            return Ok(());
        }
        let _ = self.store.clear();
        self.store.save(refresh).map_err(|e| Error::TokenSave(e.to_string()))
    }

    fn access_is_fresh(&self) -> bool {
        let (Some(_), Some(obtained)) = (&self.access, self.obtained) else {
            return false;
        };
        let leeway = Duration::from_secs(60.min(self.expires_in / 2));
        let lifetime = Duration::from_secs(self.expires_in);
        obtained.elapsed() + leeway < lifetime
    }
}
