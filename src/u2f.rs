use u2f::protocol::*;
use u2f::messages::*;
use u2f::register::*;
use once_cell::sync::OnceCell;
use std::{
    convert::{Into},
    sync::{Mutex},
};
use chrono::prelude::*;
use base64::{encode_config, URL_SAFE_NO_PAD};
use serde_derive::{Serialize};

type Result<T> = std::result::Result<T, anyhow::Error>;

static VERSION: &'static str = "U2F_V2";

pub fn make_challenge(app_id: &str, challenge_bytes: Vec<u8>) -> Challenge {
    let utc: DateTime<Utc> = Utc::now();
    Challenge {
        challenge: encode_config(&challenge_bytes, URL_SAFE_NO_PAD),
        timestamp: format!("{:?}", utc),
        app_id: app_id.to_string()
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationVerification {
    pub key_handle: String,
    pub pubkey: String,
    pub device_name: Option<String>,
}

pub fn verify_registration(app_id: String, challenge_bytes: Vec<u8>, register_data: Vec<u8>, client_data: Vec<u8>) -> Result<String> {
    let challenge = make_challenge(&app_id, challenge_bytes);
    let register_data_base64 = encode_config(&register_data, URL_SAFE_NO_PAD);
    let client_data_base64 = encode_config(&client_data, URL_SAFE_NO_PAD);
    let client = U2f::new(app_id.into());
    match client.register_response(challenge, RegisterResponse {
        registration_data: register_data_base64,
        client_data: client_data_base64,
        version: VERSION.to_string(),
    }) {
        Ok(v)=> {
            let rv = RegistrationVerification {
                key_handle: base64::encode(&v.key_handle),
                pubkey: base64::encode(&v.pub_key),
                device_name: v.device_name,
            };
            Ok(serde_json::to_string(&rv)?)
        },
        Err(e)=> Err(e.into())
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignatureVerification {
    pub counter: u8,
}

pub fn verify_signature(app_id: String, challenge_bytes: Vec<u8>, sign_data: Vec<u8>, client_data: Vec<u8>, key_handle: Vec<u8>, pub_key: Vec<u8>) -> Result<u32> {
    let challenge = make_challenge(&app_id, challenge_bytes);
    let sign_data_base64 = encode_config(&sign_data, URL_SAFE_NO_PAD);
    let client_data_base64 = encode_config(&client_data, URL_SAFE_NO_PAD);
    let key_handle_base64 = encode_config(&key_handle, URL_SAFE_NO_PAD);
    let client = U2f::new(app_id.into());
    let mut _counter: u32 = 0;
    match client.sign_response(
        challenge, 
        Registration { // here only needs pubkey and keyhandle
            key_handle: key_handle,
            pub_key: pub_key,
            attestation_cert: None,
            device_name: None,
        },
        SignResponse { // here needs client data and sig data and key_handle
            signature_data: sign_data_base64,
            client_data: client_data_base64,
            key_handle: key_handle_base64,
        },
        _counter
    ) {
        Ok(v)=> Ok(v),
        Err(e)=> Err(e.into()),
    }
}
