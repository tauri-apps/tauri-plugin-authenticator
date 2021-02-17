mod auth;

use serde::{Deserialize};

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
    Sign {
        timeout: u64, // milliseconds
        challenge: String,
        application: String,
        #[serde(rename = "keyHandle")]
        key_handle: String, // base64
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
                    Init{
                        callback,
                        error
                    }=> {
                        tauri::execute_promise(
                            webview,
                            move || {
                                Ok(auth::init_usb())
                            },
                            callback,
                            error,
                        );
                    }
                    Register{
                        timeout, 
                        challenge, 
                        application,
                        callback,
                        error,
                    }=> {
                        tauri::execute_promise(
                            webview,
                            move || {
                                auth::register(timeout, challenge, application)            
                            },
                            callback,
                            error,
                        );
                    }
                    Sign{
                        timeout, 
                        challenge, 
                        application,
                        key_handle,
                        callback,
                        error
                    }=> {
                        tauri::execute_promise(
                            webview,
                            move || {
                                auth::sign(timeout, challenge, application, key_handle)
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