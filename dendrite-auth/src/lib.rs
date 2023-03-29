pub mod dendrite {
    pub use ::dendrite_lib::*;
}

pub mod dendrite_config;

use crate::dendrite_config::{
    CredentialsAddedEvent, CredentialsRemovedEvent, KeyManagerAddedEvent, KeyManagerRemovedEvent,
    TrustedKeyAddedEvent, TrustedKeyRemovedEvent,
};
use anyhow::{anyhow, Context, Result};
use core::convert::TryFrom;
use dendrite_lib::axon_server::event::Event;
use dendrite_lib::axon_utils::{
    empty_handler_registry, event_processor, AsyncApplicableTo, AxonServerHandle,
    TheHandlerRegistry, TokenStore, WorkerControl,
};
use dendrite_lib::register;
use dendrite_macros;
use jwt::algorithm::AlgorithmType::Rs256;
use jwt::token::Unverified;
use jwt::{AlgorithmType, Error, Header, Token, VerifyWithKey};
use lazy_static::lazy_static;
use log::{debug, error, warn};
use prost::Message;
use rsa::pkcs8::PrivateKeyInfo;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey};
use serde_json::Value;
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};
use sshkeys;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tonic;

const SEPARATOR: &str = ".";

struct AuthSettings {
    key_managers: HashMap<String, String>,
    trusted_keys: HashMap<String, String>,
    credentials: HashMap<String, String>,
    private_key: Option<RsaPrivateKey>,
    private_key_name: String,
}

#[derive(Clone)]
struct AuthQueryModel {
    auth_settings: Arc<Mutex<AuthSettings>>,
}

#[tonic::async_trait]
impl TokenStore for AuthQueryModel {
    async fn store_token(&self, _token: i64) {}

    async fn retrieve_token(&self) -> Result<i64> {
        Ok(0)
    }
}

lazy_static! {
    static ref AUTH: AuthQueryModel = AuthQueryModel {
        auth_settings: Arc::new(Mutex::new(AuthSettings {
            key_managers: HashMap::new(),
            trusted_keys: HashMap::new(),
            credentials: HashMap::new(),
            private_key: None,
            private_key_name: "".to_string(),
        }))
    };
}

/// Handles auth events.
///
/// Constructs an event handler registry and delegates to function `event_processor`.
pub async fn process_events(axon_server_handle: AxonServerHandle, worker_control: WorkerControl) {
    if let Err(e) = internal_process_events(axon_server_handle, worker_control).await {
        error!("Error while handling commands: {:?}", e);
    }
    debug!("Stopped handling auth events");
}

async fn internal_process_events(
    axon_server_handle: AxonServerHandle,
    worker_control: WorkerControl,
) -> Result<()> {
    let mut event_handler_registry: TheHandlerRegistry<
        AuthQueryModel,
        Event,
        Option<AuthQueryModel>,
    > = empty_handler_registry();

    register!(event_handler_registry, handle_trusted_key_added_event)?;
    register!(event_handler_registry, handle_trusted_key_removed_event)?;
    register!(event_handler_registry, handle_key_manager_added_event)?;
    register!(event_handler_registry, handle_key_manager_removed_event)?;
    register!(event_handler_registry, handle_credentials_added_event)?;
    register!(event_handler_registry, handle_credentials_removed_event)?;

    event_processor(
        axon_server_handle,
        AUTH.clone(),
        event_handler_registry,
        worker_control,
    )
    .await
    .context("Error while handling commands")
}

#[dendrite_macros::event_handler]
async fn handle_trusted_key_added_event(
    command: TrustedKeyAddedEvent,
    _query_model: AuthQueryModel,
) -> Result<()> {
    if let Some(public_key) = command.public_key.clone() {
        unchecked_set_public_key(public_key)?
    }
    Ok(())
}

#[dendrite_macros::event_handler]
async fn handle_trusted_key_removed_event(
    command: TrustedKeyRemovedEvent,
    _query_model: AuthQueryModel,
) -> Result<()> {
    remove_public_key(&command.name)
}

#[dendrite_macros::event_handler]
async fn handle_key_manager_added_event(
    command: KeyManagerAddedEvent,
    _query_model: AuthQueryModel,
) -> Result<()> {
    if let Some(public_key) = command.public_key.clone() {
        unchecked_set_key_manager(public_key)?
    }
    Ok(())
}

#[dendrite_macros::event_handler]
async fn handle_key_manager_removed_event(
    command: KeyManagerRemovedEvent,
    _query_model: AuthQueryModel,
) -> Result<()> {
    remove_key_manager(&command.name)
}

#[dendrite_macros::event_handler]
async fn handle_credentials_added_event(
    command: CredentialsAddedEvent,
    _query_model: AuthQueryModel,
) -> Result<()> {
    if let Some(credentials) = command.credentials.clone() {
        unchecked_set_credentials(credentials)?
    }
    Ok(())
}

#[dendrite_macros::event_handler]
async fn handle_credentials_removed_event(
    command: CredentialsRemovedEvent,
    _query_model: AuthQueryModel,
) -> Result<()> {
    remove_credentials(&command.identifier)
}

pub fn set_private_key(key_name: String, pem_string: String) -> Result<()> {
    let private_key_info = PrivateKeyInfo::try_from(pem_string.as_bytes())?;
    let private_key = RsaPrivateKey::try_from(private_key_info)?;
    // TODO: verify that the private key matches the public key with the same name
    let mut auth_settings = AUTH
        .auth_settings
        .lock()
        .map_err(|e| anyhow!(e.to_string()))?;
    auth_settings.private_key_name = key_name;
    auth_settings.private_key = Some(private_key);
    Ok(())
}

pub fn unchecked_set_public_key(public_key: dendrite_config::PublicKey) -> Result<()> {
    let mut auth_settings = AUTH
        .auth_settings
        .lock()
        .map_err(|e| anyhow!(e.to_string()))?;
    auth_settings
        .trusted_keys
        .insert(public_key.name, public_key.public_key);
    Ok(())
}

pub fn unchecked_set_key_manager(public_key: dendrite_config::PublicKey) -> Result<()> {
    let mut auth_settings = AUTH
        .auth_settings
        .lock()
        .map_err(|e| anyhow!(e.to_string()))?;
    auth_settings
        .key_managers
        .insert(public_key.name, public_key.public_key);
    Ok(())
}

fn unchecked_set_credentials(credentials: dendrite_config::Credentials) -> Result<()> {
    let mut auth_settings = AUTH
        .auth_settings
        .lock()
        .map_err(|e| anyhow!(e.to_string()))?;
    auth_settings
        .credentials
        .insert(credentials.identifier, credentials.secret);
    Ok(())
}

fn remove_public_key(name: &str) -> Result<()> {
    let mut auth_settings = AUTH
        .auth_settings
        .lock()
        .map_err(|e| anyhow!(e.to_string()))?;
    auth_settings.trusted_keys.remove(name);
    Ok(())
}

fn remove_key_manager(name: &str) -> Result<()> {
    let mut auth_settings = AUTH
        .auth_settings
        .lock()
        .map_err(|e| anyhow!(e.to_string()))?;
    auth_settings.key_managers.remove(name);
    Ok(())
}

fn remove_credentials(identifier: &str) -> Result<()> {
    let mut auth_settings = AUTH
        .auth_settings
        .lock()
        .map_err(|e| anyhow!(e.to_string()))?;
    auth_settings.credentials.remove(identifier);
    Ok(())
}

struct AuthPublicKey(rsa::RsaPublicKey);

impl jwt::VerifyingAlgorithm for AuthPublicKey {
    fn algorithm_type(&self) -> AlgorithmType {
        Rs256
    }

    fn verify_bytes(&self, header: &str, claims: &str, signature: &[u8]) -> Result<bool, Error> {
        let mut hasher = Sha256::new();
        hasher.update(header.as_bytes());
        sha2::Digest::update(&mut hasher, SEPARATOR.as_bytes());
        sha2::Digest::update(&mut hasher, claims.as_bytes());
        let hashed: &[u8] = &hasher.finalize_fixed();
        debug!("Verify signature: {:?}: {:?}", signature, hashed);
        let padding = PaddingScheme::new_pkcs1v15_sign::<Sha256>();
        self.0.verify(padding, hashed, signature).map_err(|e| {
            warn!("Error during verification of JWT: {:?}", e);
            Error::InvalidSignature
        })?;
        Ok(true)
    }
}

pub fn verify_jwt(jwt: &str) -> Result<HashMap<String, Value>> {
    let unverified: Token<Header, HashMap<String, Value>, Unverified> =
        Token::parse_unverified(jwt)?;
    let header = unverified.header();
    debug!("Header: {:?}", header);
    if let Some(ref key_id) = header.key_id {
        let auth_settings = AUTH
            .auth_settings
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?;
        if let Some(key) = auth_settings.trusted_keys.get(key_id) {
            debug!("Key: {:?}", key);
            let key = format!("ssh-rsa {}", key);
            let public_key = sshkeys::PublicKey::from_string(&key)?;
            debug!("SSH public key: {:?}", public_key);
            if let sshkeys::PublicKey {
                kind: sshkeys::PublicKeyKind::Rsa(sshkeys::RsaPublicKey { n, e }),
                ..
            } = public_key
            {
                let n = rsa::BigUint::from_bytes_be(&n);
                let e = rsa::BigUint::from_bytes_be(&e);
                let public_key = rsa::RsaPublicKey::new(n, e)?;
                debug!("RSH public key: {:?}", public_key);
                let verified = unverified.verify_with_key(&AuthPublicKey(public_key))?;
                return Ok(verified.claims().clone());
            }
        }
    }
    Err(anyhow!("Invalid signature"))
}
