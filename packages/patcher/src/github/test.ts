import { PatcherFile, PatcherNodeType } from '@patcher/types';

export const testWorkflow = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `# This is a basic workflow to help you get started with Actions

name: CI

# Controls when the action will run. Triggers the workflow on push or pull request
# events but only for the main branch
on: [push, pull_request]

# A workflow run is made up of one or more jobs that can run sequentially or in parallel
jobs:
  # This workflow contains a single job called "build"
  build-and-test:
    runs-on: ubuntu-latest

    # Steps represent a sequence of tasks that will be executed as part of the job
    steps:
      # Checks-out your repository under $GITHUB_WORKSPACE, so your job can access it
      # Checks out a copy of your repository on the ubuntu-latest machine
      - name: Checkout code
        uses: actions/checkout@v2

      - uses: cachix/install-nix-action@v12
        with:
          nix_path: nixpkgs=channel:nixos-unstable

      - name: build holochain
        run: |
          nix-env -iA cachix -f https://cachix.org/api/v1/install
          cachix use holochain-ci
          cd $GITHUB_WORKSPACE
          nix-shell . --run "npm i && npm run build:happ"

      - name: test holochain
        run: |
            cd $GITHUB_WORKSPACE
            nix-shell . --run "npm test"

        - name: build & package UI
        run: |
            cd $GITHUB_WORKSPACE
            nix-shell . --run "npm run package"
`,
});
