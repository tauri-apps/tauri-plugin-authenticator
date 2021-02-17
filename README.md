# tauri-plugin-authenticator
An official Tauri plugin for using a yubikey in your Tauri App
# Tauri Plugin Authenticator
![Test](https://github.com/tauri-apps/tauri-plugin-authenticator/workflows/Test/badge.svg)

This plugin provides a "classical" Tauri Plugin Interface to the wonderful mozilla authenticator.rs

## Note:
This plugin is currently written to comply with the pre-beta version of Tauri. It will change when the beta release candidate is published and is currently instable.

## Architecture
This repo shape might appear to be strange, but it is really just a hybrid Rust / Typescript project that recommends a specific type of consumption, namely using GIT as the secure distribution mechanism, and referencing specific unforgeable git hashes. Of course, it can also be consumed via Cargo and NPM.

### `/src`
Rust source code that contains the plugin definition and Authenticator features.

### `/webview-src`
Typescript source for the /dist folder that provides an API to interface with the rust code.

### `/webview-dist`
Tree-shakeable transpiled JS to be consumed in a WRY webview.

### `/bindings`
Forthcoming tauri bindings to other programming languages, like DENO.

## Installation
There are three general methods of installation that we can recommend.
1. Pull sources directly from Github using git tags / revision hashes (most secure, good for developement, shown below)
2. Git submodule install this repo in your tauri project and then use `file` protocol to ingest the source
3. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)

For more details and usage see [the svelte demo](examples/svelte-app/src/App.svelte). Please note, below in the dependencies you can also lock to a revision/tag in both the `Cargo.toml` and `package.json`

### RUST
`src-tauri/Cargo.toml`
```yaml
[dependencies.tauri-plugin-authenticator]
git = "https://github.com/tauri-apps/tauri-plugin-authenticator"
tag = "v0.1.0"
#branch = "main"
```

Use in `src-tauri/src/main.rs`:
```rust
use tauri_authenticator::TauriAuthenticator;

fn main() {
    tauri::AppBuilder::new()
        .plugin(TauriAuthenticator {})
        .build()
        .run();
}
```

### WEBVIEW
`Install`
```
npm install github:tauri-apps/tauri-plugin-authenticator-api#v0.1.0
# or
yarn add github:tauri-apps/tauri-plugin-authenticator-api#v0.1.0
```

`package.json`
```json
  "dependencies": {
    "tauri-plugin-authenticator-api": "tauri-apps/tauri-plugin-authenticator-api#v0.1.0",
```

Use within your JS/TS:
```
  import { Authenticator } from 'tauri-plugin-authenticator-api'
```

# License
MIT / Apache-2.0
