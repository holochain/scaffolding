pub fn workspace_package_json(
    app_name: String,
    ui_package_name: String,
    web_app_workdir_path: String,
    app_workdir_path: String,
) -> String {
    format!(
        r#"{{
      "name": "{}-dev",
      "private": true,
      "workspaces": [
        "ui",
        "tests"
      ],
      "scripts": {{
        "start": "npm run network 2",
        "network": "hc s clean && npm run build:happ && concurrently-repeat \"npm run start:agent\"",
        "start:agent": "cross-env HC_PORT=$(port) concurrently -k \"npm run start:happ\" \"sleep 5 && npm run start -w {}\"",
        "test": "hc-scaffold pack web-app workdir && npm t -w tests",
        "start:happ": "concurrently \"RUST_LOG=warn echo \"pass\" | hc s --piped generate {}/{}.happ --run=$HC_PORT -a {} network mdns\" \"npm run playground\"",
        "package": "npm run build:happ && npm run package -w {} && hc web-app pack {}",
        "build:happ": "npm run build:zomes && hc-scaffold pack app {}",
        "build:zomes": "CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown",
        "playground": "run-singleton \"holochain-playground\""
      }},
      "devDependencies": {{
        "@holochain-playground/cli": "^0.0.11",
        "concurrently": "^6.2.1",
        "concurrently-repeat": "^0.0.1",
        "cross-env": "^7.0.3",
        "new-port-cli": "^1.0.0",
        "rimraf": "^3.0.2",
        "run-singleton-cli": "^0.0.5"
      }},
      "engines": {{
        "npm": ">=7.0.0"
      }}
  }}"#,
        app_name,
        ui_package_name,
        app_workdir_path,
        app_name,
        app_name,
        ui_package_name,
        web_app_workdir_path,
        app_workdir_path,
    )
}
