### Manually testing the Scaffolding tool

#### Ensuring you're using the right Scaffolding version

With the Scaffolding repository checked out, switch to the Git revision you want to test.

Start the Nix development environment for the Scaffolding tool using:

```
nix develop --override-input "versions/scaffolding" .
```

This will give you a shell which includes the Scaffolding built from your local repository.

Stay in the current Nix shell and change directory to somewhere that you want to put your test project. 
For example, `cd /tmp`. You're now ready to scaffold a new app with `hc-scaffold web-app`.

When following the instructions that the Scaffolding outputs, ignore `nix develop` and instead run:

```
nix develop --override-input "versions/scaffolding" <path-to-local-scaffolding-clone>
```

Which will ensure that you keep using the updated Scaffolding tool inside the scaffolded hApp environment.

Now you can proceed with testing your changes as needed.
