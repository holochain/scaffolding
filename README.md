# RAD Scaffolding tools for Holochain applications

RAD tools to enable quick scaffolding of Holochain application.

# Usage

```bash
npm init @holochain
```

This will open a tab in your browser that will guide you through the process of scaffolding a Holochain app.

# Development Setup

## Structure

The Holochain scaffolding tools are structured as an Yarn monorepo, to allow for composability and decoupling of its building blocks.

Packages:

- `@holochain/scaffolding` (located in `packages/scaffolding`): types, elements and vanilla JS functions to help design and generate Holochain applications.
- `@holochain-scaffolding/ui` (located in `packages/client`): Vue app that serves as the client for `@holochain/create`.
- `@holochain/create` (located in `packages/create`): aggregator package that can be executed to scaffold fully working Holochain apps.

## Usage

### Installing

From the root folder of the repository, run:

```bash
yarn
```

### Testing

From the root folder of the repository, run:

```bash
npm test
```

This will scaffold a fully working holochain application and run its tests.

### Starting

From the root folder of the repository, run:

```bash
npm start
```

### Building @holochain/create

From the root of the repository, run:

```bash
npm run build
```
