mod auth;
mod u2f;

use serde::Deserialize;

pub struct TauriAuthenticator;

#[derive(Deserialize)]
#[serde(tag = "cmd")]
enum AuthenticatorCmd {
    Init {
        callback: String,
        error: String,
    },
    Register {
        timeout: u64, // milliseconds
        challenge: String,
        application: String,
        callback: String,
        error: String,
    },
    VerifyRegistration {
        challenge: String,
        application: String,
        #[serde(rename = "registerData")]
        register_data: String,
        #[serde(rename = "clientData")]
        client_data: String,
        callback: String,
        error: String,
    },
    Sign {
        timeout: u64, // milliseconds
        challenge: String,
        application: String,
        #[serde(rename = "keyHandle")]
        key_handle: String, // base64
        callback: String,
        error: String,
    },
    VerifySignature {
        challenge: String,
        application: String,
        #[serde(rename = "signData")]
        sign_data: String,
        #[serde(rename = "clientData")]
        client_data: String,
        #[serde(rename = "keyHandle")]
        key_handle: String, // base64
        pubkey: String, // base64
        callback: String,
        error: String,
    },
}

impl tauri::plugin::Plugin for TauriAuthenticator {
    fn extend_api(&self, webview: &mut tauri::Webview<'_>, payload: &str) -> Result<bool, String> {
        use AuthenticatorCmd::*;
        match serde_json::from_str(payload) {
            Err(e) => Err(e.to_string()),
            Ok(command) => {
                match command {
                    Init { callback, error } => {
                        tauri::execute_promise(
                            webview,
                            move || {
                                auth::init_usb();
                                Ok(())
                            },
                            callback,
                            error,
                        );
                    }
                    Register {
                        timeout,
                        challenge,
                        application,
                        callback,
                        error,
                    } => {
                        tauri::execute_promise(
                            webview,
                            move || auth::register(application, timeout, challenge),
                            callback,
                            error,
                        );
                    }
                    VerifyRegistration {
                        challenge,
                        application,
                        register_data,
                        client_data,
                        callback,
                        error,
                    } => {
                        tauri::execute_promise(
                            webview,
                            move || {
                                u2f::verify_registration(
                                    application,
                                    challenge,
                                    register_data,
                                    client_data,
                                )
                            },
                            callback,
                            error,
                        );
                    }
                    Sign {
                        timeout,
                        challenge,
                        application,
                        key_handle,
                        callback,
                        error,
                    } => {
                        tauri::execute_promise(
                            webview,
                            move || auth::sign(application, timeout, challenge, key_handle),
                            callback,
                            error,
                        );
                    }
                    VerifySignature {
                        challenge,
                        application,
                        sign_data,
                        client_data,
                        key_handle,
                        pubkey,
                        callback,
                        error,
                    } => {
                        tauri::execute_promise(
                            webview,
                            move || {
                                u2f::verify_signature(
                                    application,
                                    challenge,
                                    sign_data,
                                    client_data,
                                    key_handle,
                                    pubkey,
                                )
                            },
                            callback,
                            error,
                        );
                    }
                };
                Ok(true)
            }
        }
    }
}
