use crate::versions::{holochain_client_version, tryorama_version};

pub fn tryorama_package_json() -> String {
    format!(
        r#"{{
  "name": "tests",
  "version": "0.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {{
    "test": "foreach -g \"**/*.test.ts\" -X \"NODE_NO_WARNINGS=1 ts-node-test #{{path}}\" --no-C"
  }},
  "author": "",
  "license": "CAL-1.0",
  "dependencies": {{
    "@msgpack/msgpack": "^2.7.0",
    "@holochain/client": "{}",
    "@holochain/tryorama": "{}",
    "foreach-cli": "^1.8.1",
    "ts-node-test": "^0.2.0"
  }},
  "type": "module"
}}"#,
        holochain_client_version(),
        tryorama_version()
    )
}
