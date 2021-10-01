# RAD Scaffolding tools for Holochain applications

RAD tools to enable quick scaffolding of Holochain application.

# Usage

```bash
npm init @holochain
```

This will open a tab in your browser that will guide you through the process of scaffolding a Holochain app.

# Development Setup

## Environment

This package requires NPM v7.14 or later.

You can use the `default.nix` included in this repository for a quick setup.

## Structure

`@holochain/create` is structured as an NPM monorepo (available from NPM v7), to allow for composability and decoupling of its building blocks.

Workspaces:

- `@holochain/scaffolding-ui` (located in `packages/client`): Vue app that serves as the client for `@holochain/create`.
- `@holochain/scaffolding-generators` (located in `packages/generators`): vanilla JS functions that can generate Holochain code.
- `@holochain/create` (located in `packages/create`): aggregator package that can be executed to scaffold fully working Holochain apps.

## Usage

### Installing

From the root folder of the repository, run:

```bash
npm install
```

### Starting

From the root folder of the repository, run:

```bash
npm start
```

### Building @holochain/create

Go into `packages/server` and run:

```bash
npm run build
```
