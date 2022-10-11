pub fn tryorama_package_json(tryorama_version: String) -> String {
    format!(
        r#"{{
  "name": "tests",
  "version": "0.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {{
    "test": "TRYORAMA_LOG_LEVEL=info node --test src"
  }},
  "author": "",
  "license": "CAL-1.0",
  "dependencies": {{
    "@msgpack/msgpack": "^2.7.0",
    "@holochain/client": "^0.9.2",
    "@holochain/tryorama": "{}"
  }},
  "type": "module"
}}"#,
        tryorama_version
    )
}
