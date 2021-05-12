mod auth;
mod u2f;

use tauri::{plugin::Plugin, Invoke, Params};

#[tauri::command]
fn init() {
    auth::init_usb();
}

#[tauri::command]
fn register(timeout: u64, challenge: String, application: String) -> Result<String, String> {
    auth::register(application, timeout, challenge).map_err(|e| e.to_string())
}

#[tauri::command]
fn verify_registration(
    challenge: String,
    application: String,
    register_data: String,
    client_data: String,
) -> Result<String, String> {
    u2f::verify_registration(application, challenge, register_data, client_data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn sign(
    timeout: u64,
    challenge: String,
    application: String,
    key_handle: String,
) -> Result<String, String> {
    auth::sign(application, timeout, challenge, key_handle).map_err(|e| e.to_string())
}

#[tauri::command]
fn verify_signature(
    challenge: String,
    application: String,
    sign_data: String,
    client_data: String,
    key_handle: String,
    pubkey: String,
) -> Result<u32, String> {
    u2f::verify_signature(
        application,
        challenge,
        sign_data,
        client_data,
        key_handle,
        pubkey,
    )
    .map_err(|e| e.to_string())
}

pub struct TauriAuthenticator<P: Params> {
    invoke_handler: Box<dyn Fn(Invoke<P>) + Send + Sync>,
}

impl<P: Params> Default for TauriAuthenticator<P> {
    fn default() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![
                init,
                register,
                verify_registration,
                sign,
                verify_signature
            ]),
        }
    }
}

impl<P: Params> Plugin<P> for TauriAuthenticator<P> {
    fn name(&self) -> &'static str {
        "authenticator"
    }

    fn extend_api(&mut self, invoke: Invoke<P>) {
        (self.invoke_handler)(invoke)
    }
}
