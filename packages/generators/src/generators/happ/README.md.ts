import { HappDefinition } from '../../types';

export default (happ: HappDefinition) => `
# ${happ.name}

## Environment Setup

1. Install the holochain dev environment (only nix-shell is required): https://developer.holochain.org/docs/install/
2. Enable Holochain cachix with:

\`\`\`bash
nix-env -iA cachix -f https://cachix.org/api/v1/install
cachix use holochain-ci
\`\`\`

3. Clone this repo and \`cd\` inside of it.
4. Enter the nix shell by running this in the root folder of the repository: 

\`\`\`bash
nix-shell
npm install
\`\`\`

This will install all the needed dependencies in your local environment, including \`holochain\`, \`hc\` and \`npm\`.

## Building the DNA

- Build the DNA (assumes you are still in the nix shell for correct rust/cargo versions from step above):

\`\`\`bash
npm run build:happ
\`\`\`

## Running the DNA tests

\`\`\`bash
npm run test
\`\`\`

## UI

To test out the UI:

\`\`\` bash
npm run start
\`\`\`

## Package

To package the web happ:

\`\`\` bash
npm run package
\`\`\`

You'll have the \`${happ.name}.webhapp\` in \`workdir\`. You will also have its subcomponent \`${happ.name}.happ\` in the same folder\`.

## Releasing

At every commit in the \`main\` branch, a release process will be triggered. It will create a draft release with your app packaged in the \`.webhapp\` file, ready to be installed in the launcher.

You can customize this behaviour by going into \`.github/workflows/release.yml\`.

## Documentation

We are using this tooling:

- [NPM Workspaces](https://docs.npmjs.com/cli/v7/using-npm/workspaces/): npm v7's built-in monorepo capabilities.
- [hc](https://github.com/holochain/holochain/tree/develop/crates/hc): Holochain CLI to easily manage Holochain development instances.
- [@holochain/tryorama](https://www.npmjs.com/package/@holochain/tryorama): test framework.
- [@holochain/conductor-api](https://www.npmjs.com/package/@holochain/conductor-api): client library to connect to Holochain from the UI.
`;
