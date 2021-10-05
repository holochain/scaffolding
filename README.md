# RAD Scaffolding tools for Holochain applications

RAD tools to enable quick scaffolding of Holochain application.

# Usage

NOTE: you will need [Node.js](https://nodejs.org/en/) installed to use this tool.

## from the NPM package

The RAD tools is packaged and available in NPM registry [here](https://www.npmjs.com/package/@holochain/create). To use the RAD tools of the specific version run the following command:

```bash
npx @holochain/create@0.0.16
```

## local from the repo

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
