# Manually Testing the Scaffolding Tool

## Testing with the Current Local Version

To test the current local version of the Scaffolding tool:

1. Check out the Scaffolding repository and switch to the Git revision you want to test.

2. Start the Nix development environment:

    ```shell
    nix develop
    ```

    This shell includes the Scaffolding tool built from your current local repository.

3. Change to a directory where you want to create your test project, e.g.:

    ```shell
    cd /tmp
    ```

4. Scaffold a new app:

    ```shell
    hc-scaffold web-app
    ```

5. Follow the Scaffolding tool's output instructions, but skip the `nix develop` step to ensure you continue using the locally built `hc-scaffold` instead of that from `holochain/holonix`.

6. Run tests or launch your hApp using `nix develop` as needed. This shell contians a `hc-scaffold` built from `holochain/holonix` alongside other nix packages needed for local development of your hApp such as node.js and npm.

## Testing with Different Holochain Versions

To test your hApp against a different version of Holochain:

1. Override the Holochain input when entering the Nix shell:

    ```shell
    nix develop --override-input holonix/holochain github:holochain/holochain/<tag>
    ```

    Replace `<tag>` with the desired Holochain version, e.g., `holochain-0.4.0-dev.22`.

2. Proceed with scaffolding and testing as described above, skipping the `nix develop` step in the Scaffolding output.

## Overriding Other Inputs

You can override other inputs like `lair-keystore` similarly:

```shell
nix develop --override-input holonix/lair-keystore github:holochain/lair/<tag>
```

Ensure you're using the correct version tags. Refer to the [parent flake](https://github.com/holochain/holonix/blob/main/flake.nix) for the latest input versions.

## Notes

-   Always verify the version of tools you're using with commands like `hc-scaffold --version` or `holochain --version`.
-   Multiple overrides can be combined in a single command if needed.
-   These overrides are temporary and only apply to the current Nix shell session.