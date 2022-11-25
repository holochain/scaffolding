# Holochain Scaffolding CLI

CLI to easily generate and edit holochain apps.

## Install

Install the CLI globally with this command.

```bash
cargo install holochain_scaffolding_cli
```

In the near future the scaffolding tool is going to be integrated together with holonix and the overall holochain development environment.

Until then, you can just install it from crates to get a preview of its functionality.


## Usage

```bash
# Scaffold an empty web-app
hc-scaffold web-app forum

cd forum

# Scaffold a dna inside the newly scaffolded app
hc-scaffold dna forum

# Scaffold a zome inside the newly scaffolded dna
hc-scaffold zome posts

# Scaffold an entry-type inside the newly scaffolded zome
hc-scaffold entry-type post

# Scaffold an index for the newly scaffolded entry-type
hc-scaffold index global all_posts

# Scaffold a new link-type
hc-scaffold link-type
```

## Documentation

See the [docs.rs documentation](https://docs.rs/holochain_scaffolding_cli) to learn how to use and create custom templates.
