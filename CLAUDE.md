# Upgrading Holochain versions

This repo is a code generator: it writes Rust (hdi/hdk) source into scaffolded
apps using `syn`/`quote` AST manipulation (not just Handlebars templates), so
bumping the pinned Holochain version can require updating Rust code in this
repo, not just a version string.

If the target release looks like it dropped a helper or introduced a logic
gap with no replacement (not just a rename), don't paper over it with a
workaround — flag it and let the user decide. This is expected during RC
cycles; upstream is often still actively fixing things.

## 1. Where the versions live

`src/versions.rs` holds five constants:

- `HOLOCHAIN_VERSION` (crates.io `holochain`)
- `HDI_VERSION` (crates.io `hdi`)
- `HDK_VERSION` (crates.io `hdk`)
- `HOLOCHAIN_CLIENT_VERSION` (npm `@holochain/client`)
- `HC_SPIN_VERSION` (npm `@holochain/hc-spin`)

Find the matching set for a target Holochain release from the
`holochain/holochain` GitHub repo tags (`holochain-X.Y.Z`, `hdi-X.Y.Z`,
`hdk-X.Y.Z`) for the three Rust crates, and the npmjs listings for the two
npm packages — verify each independently, since the Rust crates and the npm
packages come from different repos and don't necessarily move in lockstep.

Two more Holochain crates are pinned in this repo's **own** `Cargo.toml`
(not in `src/versions.rs`, and easy to miss):

- `holochain_types` — the CLI reads and writes `dna.yaml`/`happ.yaml` through
  `DnaManifest`/`AppManifest`/`ZomeManifest`
- `mr_bundle` — the `Manifest` trait behind those, plus
  `ScaffoldError::MrBundleError`

These must be bumped to the same release as `HOLOCHAIN_VERSION`. They are not
just build hygiene: the manifest *format* the tool emits comes from these
crates, so leaving them behind can silently generate a `dna.yaml` that the
target conductor won't load. Check the crates.io versions independently —
`mr_bundle` in particular has published `-dev`/`-rc` lines that lag the main
release train.

This repo does **not** pin `holochain_client` as a Rust crate, and it never
emits `holochain_types` or `mr_bundle` into a scaffolded app — generated
workspaces only get `hdi`, `hdk`, `serde`, `holochain_serialized_bytes`,
`holochain` and `tokio` (see `add_common_zome_dependencies_to_workspace_cargo()`
in `src/scaffold/zome.rs`). If a reference app has more than that in its
workspace `Cargo.toml`, that's app-specific customization — don't add it here
on their account.

### The holonix ref

`holonix.url = "github:holochain/holonix?ref=..."` appears in **four** places,
and they must all say the same thing:

- `flake.nix` — this repo's own dev shell / CI
- `src/scaffold/app/nix.rs` (`flake_nix()`) — the flake written into every
  scaffolded app
- `src/cli/custom-template/flake.nix` and `src/cli/custom-template/README.md`
  — the custom-template scaffold and its docs

holonix keeps a `main-X.Y` branch per released Holochain line and lets `main`
move on to the next dev cycle. While this repo is tracking a dev/rc cycle the
ref floats on `main`; once the target line has released, pin all four to
`main-X.Y` so a scaffolded app's dev shell can't drift onto the next major
while its `Cargo.toml` is pinned to the old one. Precedent:
`e94a1d81 build: pin holonix to main-0.6` (pin) and
`66bad51f feat: Upgrade to 0.6` (unpin back to `main`).

Right after the branch is cut, `main` and `main-X.Y` still resolve to the same
Holochain, so switching looks like a no-op in the dev shell even though the
`flake.lock` revs differ. That is expected and is not evidence the pin is
unnecessary — the point is what `main` does *next*. Compare the two branches'
`flake.nix` `holochain.url` ref (not their head revs) to confirm they still
agree today.

## 2. Where the HDI/HDK API surface is hard-coded

The `validate()` callback and its match arms are built programmatically via
`syn`/`quote`, so breaking changes to the `hdi::flat_op` module (`FlatOp`,
`OpEntry`, `OpUpdate`, `OpDelete`, `OpLink`, `OpRecord`, `OpActivity`,
`TypedAction`) or to the `Action`/`ActionData` shape need matching edits in:

- `src/scaffold/zome/integrity.rs` — `initial_lib_rs()`: the base `validate()`
  skeleton generated for a brand-new integrity zome (before any entry/link
  types exist), plus the always-static `AgentActivity`/`CreateAgent` arm.
- `src/scaffold/entry_type/integrity.rs` — `render_entry_definition_file()`
  (the `validate_create_X`/`validate_update_X`/`validate_delete_X` function
  signatures) and `add_entry_type_to_validation_arms()` + its `handle_*_arm`
  helpers (the logic that finds the right match arm in an existing
  `validate()` and inserts a new entry-type arm into it).
- `src/scaffold/link_type/integrity.rs` — same idea for link types:
  `validate_referenceable()`, the `validate_create_link_X`/
  `validate_delete_link_X` signatures in `add_link_type_to_integrity_zome()`,
  `add_link_type_to_validation_arms()`, and `add_link_type_signals()` /
  `signal_action_match_arms()` (the coordinator-side `Signal::LinkCreated`/
  `LinkDeleted` codegen).
- `src/scaffold/entry_type/coordinator.rs` — `signal_action_match_arms()`
  (the coordinator-side `Signal::EntryCreated`/`Updated`/`Deleted` codegen)
  and the `match action.hashed.content...` skeleton it shares with the
  file above.
- `src/scaffold/zome/coordinator.rs` — `initial_cargo_toml()`: the
  per-coordinator-zome dev-dependency feature list for the `holochain` crate
  (used only for `#[tokio::test]` sweettest-based zome tests). Check this
  against the real `holochain` crate's `[features]` table
  (`crates/holochain/Cargo.toml` in `holochain/holochain` at the target tag)
  — feature names do get renamed or removed between releases, so a feature
  list that compiled against the old target version isn't guaranteed to
  still be valid.

None of the above five files have unit tests that exercise the generated
`validate()`/`signal_action` content end-to-end (the existing
`src/templates/entry_type/tests/*` and `src/templates/link_type/tests/*`
tests only cover the *coordinator test file* templates, not the integrity
zome). **Always verify by actually running the CLI**, not by trusting
`cargo test` — see §4.

## 3. Finding the target API shape

Don't guess the new API from memory. Two reliable sources, in order of
preference:

1. **A reference app's upgrade PR**, if the user provides one (e.g. an app
   scaffolded by this tool that someone has since hand-upgraded). Diff its
   `dnas/*/zomes/{integrity,coordinator}/*/src/*.rs` between the old and new
   version — this shows the exact generated-code shape to target. Prefer
   fetching the **final file state** at the PR's head branch (via `gh api
   repos/<org>/<repo>/contents/<path>?ref=<branch>` + base64 decode) over
   reconstructing it by hand from a diff — less error-prone for large
   rewrites.
2. **The `hdi` crate source itself**, at the exact target tag, e.g.:
   `gh api repos/holochain/holochain/contents/crates/hdi/src/flat_op.rs?ref=hdi-X.Y.Z`
   (and its `flat_op/` submodule: `flat_op_entry.rs`, `flat_op_record.rs`,
   `flat_op_activity.rs`, `typed_action.rs`). This is the ground truth for
   exact field names/types and is worth cross-checking even when you have a
   reference app, since one app's usage may not exercise every code path
   this tool needs to generate (e.g. the "no entry/link types yet" skeleton
   state, which no scaffolded app ever ships with).

## 4. Verifying the change

**`run_test.sh` at the repo root is what CI actually runs (`testbuild` job)
and is the authoritative acceptance test — it's much more thorough than any
hand-rolled check.** It scaffolds a full "forum" app through the real CLI
binary (svelte template, `hc-scaffold web-app/dna/zome/entry-type/
collection/link-type`), covering scenarios a quick manual pass easily
misses: `--reference-entry-hash true` entries, `Enum`/`Vec`/`Option` fields,
agent-role and `:EntryHash`-suffixed link referenceables, bidirectional
links, and multiple collection types. Then it runs `npm install && npm run
test && npm run package` (JS side, plus a real wasm32 zome build) and
`cargo clippy --all -- -D warnings`. If you're not sure a manual check is
representative, this script is more likely to be right than your intuition
about what to cover — it caught a real bug (`Record::new()`'s signature
change, see below) that a narrower manual pass had missed.

CI also independently runs `cargo fmt --all -- --check`, `cargo clippy
--all-targets -- -D warnings`, and `cargo test --lib` as separate jobs — run
all of these locally before considering a change to this repo done, not
just `cargo build`/`cargo test`. A change that only touches `syn::parse_quote!`/
`quote!` blocks (rather than the `syn::parse_str(...)` calls used to insert
match arms) can still fail `cargo fmt --all -- --check` even though the
tool's own build and tests pass — formatting is checked against the
generator's own source, not the code it emits.

For fast local iteration while developing a codegen change (not as a
substitute for `run_test.sh` before considering the change verified):

```sh
cargo build --bin hc-scaffold
BIN=target/debug/hc-scaffold

# web-app: --setup-nix has no non-interactive "no" — it always prompts
# unless the flag is passed (which then tries to shell out to `nix`).
# Either drive the prompt with a PTY (see below) or just skip web-app
# and hand-write a minimal workspace Cargo.toml + `dna`/`zome` from there.
"$BIN" -t headless web-app testapp "desc" --disable-fast-track

cd testapp
"$BIN" -t headless dna testapp
"$BIN" -t headless zome testapp --dna testapp \
  --integrity dnas/testapp/zomes/integrity \
  --coordinator dnas/testapp/zomes/coordinator

# Scaffold at least two entry types (one with a `linked_from` dependency,
# to exercise deps_validation, and one with `--reference-entry-hash true`)
# and two link types (one deletable, one not), to hit every insertion
# branch — first-arm-in-skeleton AND subsequent-arm-push both need coverage.
"$BIN" -t headless entry-type dino --dna testapp --zome testapp_integrity \
  --crud crud --reference-entry-hash false \
  --link-from-original-to-each-update false \
  --fields "name:String:TextField" --no-ui
"$BIN" -t headless entry-type nest --dna testapp --zome testapp_integrity \
  --crud cr --reference-entry-hash true \
  --fields "dino_hash:ActionHash::Dino" --no-ui
"$BIN" -t headless link-type nest dino --dna testapp --zome testapp_integrity \
  --bidirectional false --delete false --no-ui
"$BIN" -t headless link-type dino nest --dna testapp --zome testapp_integrity \
  --bidirectional false --delete true --no-ui

# Does it compile against the *actual* target crates?
cargo check -p testapp_integrity --lib   # fast, no dev-deps
cargo check -p testapp --lib             # fast, no dev-deps
cargo check --workspace --all-targets    # slow (~10+ min): pulls in the
                                          # full `holochain` dev-dependency
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

If a `Select`/`Confirm` prompt can't be avoided via flags, drive it with a
pty (plain piped stdin makes `dialoguer` redraw forever instead of erroring):

```python
# minimal pattern; see conversation history for a fuller version
import pty, subprocess, os, time
master, slave = pty.openpty()
p = subprocess.Popen(cmd, stdin=slave, stdout=slave, stderr=slave)
os.close(slave)
time.sleep(3)
os.write(master, b"\x1b[B\r")  # Down, Enter — e.g. to answer "No"
```

## 5. Things that are *not* pinned here (don't "fix" them)

- **Extra `holochain` crate features on a reference app**: this tool only
  ever enables a small, fixed feature set on the `holochain` dev-dependency
  (see `initial_cargo_toml()` in `src/scaffold/zome/coordinator.rs` for the
  current list). If a reference app's `Cargo.toml` has more features
  enabled at the workspace level than that, it's likely a hand-edit made
  after scaffolding, not something this tool generated — don't replicate it
  here just because a reference app has it.
