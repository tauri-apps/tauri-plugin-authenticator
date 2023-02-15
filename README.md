![plugin-authenticator](banner.png)

Use hardware security-keys in your Tauri App.

## Install

_This plugin requires a Rust version of at least **1.64**_

There are three general methods of installation that we can recommend.

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from Github using git tags / revision hashes (most secure)
3. Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-authenticator = "0.1"
# or through git
tauri-plugin-authenticator = { git = "https://github.com/tauri-apps/plugins-workspace", branch = "dev" }
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

> Note: Since most JavaScript package managers are unable to install packages from git monorepos we provide read-only mirrors of each plugin. This makes installation option 2 more ergonomic to use.

```sh
pnpm add https://github.com/tauri-apps/tauri-plugin-authenticator
# or
npm add https://github.com/tauri-apps/tauri-plugin-authenticator
# or
yarn add https://github.com/tauri-apps/tauri-plugin-authenticator
```

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_authenticator::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```javascript
import { Authenticator } from "tauri-plugin-authenticator-api";

const auth = new Authenticator();
auth.init(); // initialize transports

// generate a 32-bytes long random challenge
const arr = new Uint32Array(32);
window.crypto.getRandomValues(arr);
const b64 = btoa(String.fromCharCode.apply(null, arr));
// web-safe base64
const challenge = b64.replace(/\+/g, "-").replace(/\//g, "_");

const domain = "https://tauri.app";

// attempt to register with the security key
const json = await auth.register(challenge, domain);
const registerResult = JSON.parse(json);

// verify te registration was successfull
const r2 = await auth.verifyRegistration(
  challenge,
  app,
  registerResult.registerData,
  registerResult.clientData
);
const j2 = JSON.parse(r2);

// sign some data
const json = await auth.sign(challenge, app, keyHandle);
const signData = JSON.parse(json);

// verify the signature again
const counter = await auth.verifySignature(
  challenge,
  app,
  signData.signData,
  clientData,
  keyHandle,
  pubkey
);

if (counter && counter > 0) {
  console.log("SUCCESS!");
}
```

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License

Code: (c) 2015 - Present - The Tauri Programme within The Commons Conservancy.

MIT or MIT/Apache 2.0 where applicable.
