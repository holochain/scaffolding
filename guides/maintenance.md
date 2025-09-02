## Maintaining the Holochain Scaffolding

This is a guide for maintainers of the Holochain Scaffolding. It may also be useful to contributors wanting to understand how the branching and releases are being done.

### Branching strategy

New development and bug fixes should target the `develop` branch. From there, changes are back-ported to maintenance branches which are named after the version of Holochain they work with. So maintenance for the versions of Scaffolding that work with Holochain 0.1.x are done on `develop-0.1` and those that work with Holochain 0.2.x are on `develop-0.2` and so on.

There need to be some exceptions to this workflow to allow changes on maintenance branches that don't make sense to make on `develop`:

-   Bug fixes that are specific to a particular version of Holochain. This should be rare, but if Holochain were to introduce an API change between 0.1.x and 0.1.x+1 but NOT make that change for Holochain 0.2.x then the PR for Scaffolding would target `develop-0.1` directly.
-   Version bumps for `flake.nix`, `flake.lock`, `Cargo.lock` and [`versions.rs`](../src/versions.rs) files should target the maintenance branches directly.
-   Version bumps for dependencies (typically holochain dependencies) of Scaffolding in the `Cargo.toml` or to Scaffolded hApps in [`versions.rs`](../src/versions.rs). For dependencies that are still on the same version as on `develop` it would be valid to target `develop` and back-port but you'll end up with a tricky lock file merge anyway.
-   Version bumps to Scaffolding itself don't make sense on `develop` so they should be made directly on the maintenance branches.

To keep back-porting as simple as possible please try to keep changes that target `develop` for back-port separate from changes that fit into these exceptions.

### Accepting PRs

This applies to anybody reviewing PRs on the Scaffolding repository whether those PRs are internal or external.

1. Check that the change targets the correct branch, following the branching strategy above. In most cases this will be `develop`. Please also check that functional changes are being kept separate from version changes where possible.
2. Check the change and work out what versions of the Scaffolding it is relevant to. Add or update the back-port labels on the PR. These take the form `ShouldBackport/0.1`, `ShouldBackport/0.2` where the version number matches the suffix of the maintenance branch, such as `develop-0.1`, `develop-0.2`.
3. Proceed with the review and testing.

### Back-porting and releasing

For changes that get merged to `develop` with the back-port labels it is then the maintainers responsibility to the back-porting. This can be done as a batch to each relevant maintenance branch. Please mention the #xxx PR number of each original PR in the back-port PR description. Once the back-port PR has merged for a given PR, the corresponding label should be removed. This denotes the completion of the back-port so that it's easy to keep track of outstanding work to be done.

Releases can be done as needed. This may be a single PR back-port, a batch of changes, or something as small as pinning a version in the `Cargo.toml`. The release process should follow these steps:

1. Submit a PR which bumps the Scaffolding version in the `Cargo.toml` on the relevant maintenance branch. This should be merged before the release.
2. Perform any testing which needs to be done before releasing the new changes.
3. Changes on a maintenance branch such as `develop-0.1` are not yet visible to Holonix because there is a tag on the branch with the same version number such as `holochain-0.1`. This is the marker that Holonix will use to fetch the current version of Scaffolding corresponding to its Holochain version. Move this label to the tip of the maintenance branch. Sample commands are given for this below.

Moving a release tag example:

```bash
git checkout develop-0.1
git pull --tags
git tag --force holochain-0.1
git push --force origin holochain-0.1
```

There is a CI job for Holonix to periodically (every 6 hours at the time of writing) looks for changes. It will automatically update its inputs for Scaffolding and create+merge a PR on the Holochain repository. So after you have pushed the updated tag, Holonix users will have to wait for the next automatic update then do their own `nix flake update` to see the latest Scaffolding.
