pub fn tryorama_tsconfig() -> String {
    format!(
        r#"{{
  "compilerOptions": {{
    "target": "ES2017",
    "module": "ESNext",
    "moduleResolution": "node",
    "esModuleInterop": true /* Enables emit interoperability between CommonJS and ES Modules via creation of namespace objects for all imports. Implies 'allowSyntheticDefaultImports'. */,
     "allowSyntheticDefaultImports": true
  }}
}}"#
    )
}
