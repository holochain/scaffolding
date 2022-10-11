

pub fn workspace_cargo_toml() -> String {
  format!(r#"[workspace]
resolver = "2"
members = [
  "dnas/*/integrity_zomes/*",
  "dnas/*/coordinator_zomes/*",
]

[profile.dev]
opt-level = "z"

[profile.release]
opt-level = "z"
"#)
}