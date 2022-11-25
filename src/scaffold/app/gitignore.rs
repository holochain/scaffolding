pub fn gitignore() -> String {
    format!(
        r#"
/node_modules/
/dist/
/target/
/.cargo/
*.happ
*.webhapp
*.zip
*.dna
.hc*
.hc
.running
"#
    )
}
