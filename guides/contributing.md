## Contributing to the Holochain Scaffolding

### Dev environment setup

This project uses the [Holonix](https://developer.holochain.org/get-started/install-advanced/) development environment. Please ensure that you have Nix installed and configured according to that guide. You could develop for the scaffolding without it but that is the environment the project's maintainers use so you may run into build issues we don't know about.

Once you have Nix installed, use a terminal to navigate to a directory where you keep source code and clone this repository:

```bash
git clone https://github.com/holochain/scaffolding.git
```

Now enter the development shell:

```bash
cd scaffolding
nix develop
```

You should now see your terminal prompt change to look something like `[rustDev:~/source/holo/scaffolding]$`.

You are now set up, you can start making changes or check the next section to learn how to run the tests.

### Running the scaffolding tests

#### Using `cargo install`
The tests expect to be able to find the scaffolding CLI in your system path. One easy way to make this to happen is to install it directly using `cargo`:

```bash
cargo install --path .
```

You should now be able to run:

```bash
hc-scaffold --version
```

and see a version number like `holochain_scaffolding_cli 0.1.11`.

To run the tests, run the provided script for a either a given template by passing a `-t` option, specifying one of the supported templates (`lit`, `svelte`, `vue`, or `vanilla`).

```bash
./run_test.sh -t "lit"
```

or employ the `-s` option with a designated scope, such as `hello_world`, to execute tests specifically for the hello world example

```bash
./run_test.sh -s "hello_world"
```

To run unit tests in Rust using Cargo, use the following command:

```bash
cargo test
```

#### Using `nix develop` to run the tests the same way as CI

You can replicate how CI compiles and introduces `hc-scaffold` to its PATH and run the test script via an ad-hoc nix environment with the following command:

```bash
nix develop --override-input "versions/scaffolding" . .#ci --command ./run_test.sh -t "lit" # or "svelte", "vue", "vanilla"`
```
This will take some time and downloads a significant amount of data so use with caution if you have limited bandwidth!

### Contributing your changes

Once you've made your changes and checked they work for you we would appreciate it if you could contribute your changes back to this project for other people to use!

Please open a pull request in the [scaffolding repository](https://github.com/holochain/scaffolding/compare) with the `base:` branch set to `develop`.

A maintainer will respond on the pull request about which versions of Holochain your change should be used with. Usually this will be all versions. In that case, the maintainers will take care of back-porting your changes to a release branch once it has been merged. In specific cases where a bugfix is specific to a single version of Holochain a maintainer will work with you to get your changes merged directly to a release branch.
