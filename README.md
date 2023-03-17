# Holochain Scaffolding CLI

CLI to easily generate and edit holochain apps.

## Obtaining the scaffolding tool through holonix

The easiest way to start using the scaffolding tool is through holonix:

```bash
nix run github:holochain/holochain#hc-scaffold -- --version
```

Should print the version of the scaffolding tool.

## Usage

Refer to [the holochain developer instructions](https://developer.holochain.org/get-building/) to know how you can use the scaffolding tool to create your own apps.

These are the commands that you can run with the scaffolding tool inside of a holonix develop shell:

```bash
# Scaffold an example app
hc scaffold example

# Scaffold an empty web-app
hc scaffold web-app forum

cd forum

# Scaffold a dna inside the newly scaffolded app
hc scaffold dna forum

# Scaffold a zome inside the newly scaffolded dna
hc scaffold zome posts

# Scaffold an entry-type inside the newly scaffolded zome
hc scaffold entry-type post

# Scaffold a collection for the newly scaffolded entry-type
hc scaffold collection global all_posts

# Scaffold a new link-type
hc scaffold link-type
```

## Documentation

See the [docs.rs documentation](https://docs.rs/holochain_scaffolding_cli) to learn how to use and create custom templates.

## Manual installation

Install the CLI globally with this command.

```bash
cargo install holochain_scaffolding_cli
```
