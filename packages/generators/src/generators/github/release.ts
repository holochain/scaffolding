export default (happName: string) =>
  `# This is a basic workflow to help you get started with Actions

name: CI

# Controls when the action will run. Triggers the workflow on push or pull request
# events but only for the main branch
on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

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

      - name: Create tag
        id: create_tag
        run: |
          tag=$(npm version patch)
          echo "::set-output name=tag::$tag"
 
      - name: package
        run: |
          cd $GITHUB_WORKSPACE
          nix-shell . --run "npm i && npm test && npm run package"
  
      - name: Release
        uses: softprops/action-gh-release@v1
        with:
          tag_name: \${{ steps.create_tag.outputs.tag }}
          draft: true
          files: |
            ./workdir/${happName}.webhapp
  
`;
