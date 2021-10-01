# Environment

This package requires NPM v7.14 or later.

# Structure

`@holochain/create` is structured as an NPM monorepo (available from NPM v7), to allow for composability and decoupling of its building blocks.

Workspaces:

- `@holochain/scaffolding-ui` (located in `packages/client`): Vue app that serves as the client for `@holochain/create`.
- `@holochain/scaffolding-generators` (located in `packages/generators`): vanilla JS functions that can generate Holochain code.
- `@holochain/create` (located in `packages/server`): aggregator package that can be executed to scaffold fully working Holochain apps.

# Usage

## Installing

From the root folder of the repository, run:

```bash
npm install
```

## Starting

From the root folder of the repository, run:

```bash
npm start
```

## Building @holochain/create

Go into `packages/server` and run:

```bash
npm run build
```
