//! Webauthn configuration and challenge data.

use serde::{Deserialize, Serialize};

#[cfg(feature = "api-types")]
use proxmox_schema::api;

use super::IsExpired;

#[cfg_attr(feature = "api-types", api)]
/// Server side webauthn server configuration.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebauthnConfig {
    /// Relying party name. Any text identifier.
    ///
    /// Changing this *may* break existing credentials.
    pub rp: String,

    /// Site origin. Must be a `https://` URL (or `http://localhost`). Should contain the address
    /// users type in their browsers to access the web interface.
    ///
    /// Changing this *may* break existing credentials.
    pub origin: String,

    /// Relying part ID. Must be the domain name without protocol, port or location.
    ///
    /// Changing this *will* break existing credentials.
    pub id: String,
}

/// For now we just implement this on the configuration this way.
///
/// Note that we may consider changing this so `get_origin` returns the `Host:` header provided by
/// the connecting client.
impl webauthn_rs::WebauthnConfig for WebauthnConfig {
    fn get_relying_party_name(&self) -> String {
        self.rp.clone()
    }

    fn get_origin(&self) -> &String {
        &self.origin
    }

    fn get_relying_party_id(&self) -> String {
        self.id.clone()
    }
}

/// A webauthn registration challenge.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebauthnRegistrationChallenge {
    /// Server side registration state data.
    pub(super) state: webauthn_rs::RegistrationState,

    /// While this is basically the content of a `RegistrationState`, the webauthn-rs crate doesn't
    /// make this public.
    pub(super) challenge: String,

    /// The description chosen by the user for this registration.
    pub(super) description: String,

    /// When the challenge was created as unix epoch. They are supposed to be short-lived.
    created: i64,
}

impl WebauthnRegistrationChallenge {
    pub fn new(
        state: webauthn_rs::RegistrationState,
        challenge: String,
        description: String,
    ) -> Self {
        Self {
            state,
            challenge,
            description,
            created: proxmox_time::epoch_i64(),
        }
    }
}

impl IsExpired for WebauthnRegistrationChallenge {
    fn is_expired(&self, at_epoch: i64) -> bool {
        self.created < at_epoch
    }
}

/// A webauthn authentication challenge.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WebauthnAuthChallenge {
    /// Server side authentication state.
    pub(super) state: webauthn_rs::AuthenticationState,

    /// While this is basically the content of a `AuthenticationState`, the webauthn-rs crate
    /// doesn't make this public.
    pub(super) challenge: String,

    /// When the challenge was created as unix epoch. They are supposed to be short-lived.
    created: i64,
}

impl WebauthnAuthChallenge {
    pub fn new(state: webauthn_rs::AuthenticationState, challenge: String) -> Self {
        Self {
            state,
            challenge,
            created: proxmox_time::epoch_i64(),
        }
    }
}

impl IsExpired for WebauthnAuthChallenge {
    fn is_expired(&self, at_epoch: i64) -> bool {
        self.created < at_epoch
    }
}
