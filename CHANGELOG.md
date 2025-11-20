# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[[0.600.0](https://github.com/holochain/scaffolding/commits/v0.600.0)\] - 2025-11-20

### Features

- *(cli)* Point nix-shell to flake version of holonix by @steveej
- *(flake)* Use separate input for holochain-nix-versions; deps: bump holochain (#94) by @steveej in [#94](https://github.com/holochain/scaffolding/pull/94)
  - Feat(flake): use separate input for holochain-nix-versions  this makes the update behavior predictable as per the original expectations.  reflect the changes in this repo's own flake as well.  * chore: bump holochain to 0.1.5-beta-rc.1
- *(flake,nix)* Bump to 0_2; feat(ci/test): use nix to build scaffolding by @steveej
  - Adapt to upstream flake changes * bump the versions input to versions/0_2 * use nix to build scaffolding on CI, which can re-use cached   dependencies accross runs
- *(flake/devShells)* Use rustDev as base for default and add ci by @steveej
- *(github/workflows/test)* Build scaffolding with nix by @steveej
- *(nix)* Use holochain's default branch by @steveej
- Remove support for Holo enabled scaffolding by @ThetaSinner
- Remove UI references to the WebSDK by @ThetaSinner in [#512](https://github.com/holochain/scaffolding/pull/512)
- Upgrade to 0.6 by @ThetaSinner
- Add u8 field type support (#433) by @c12i in [#433](https://github.com/holochain/scaffolding/pull/433)
  - Add u8 field type
  - Represent Vec<u8> as a Uint8Array
  - Add u8 widgets
  - Fix misplaced lables
  - Fix svelte slider props
  - Ensure correct TS type is generated for Vec<u8>
  - Skip ui generation for Vec<u8> types
- Add ability to go back and modify entry type fields (#418) by @c12i in [#418](https://github.com/holochain/scaffolding/pull/418)
  - Add go back functionality for hc scaffold entry-type  * address PR comments  * Ensure the fields vec is not empty on change  * Improve text spacing  * Simplify prompt
- Further improve ci runtime (#414) by @c12i in [#414](https://github.com/holochain/scaffolding/pull/414)
  - Remove dependency to holochain  * Add rust-cache  * Add cachix nix config  * Add ci job concurrency options  * Update cachix configurations  * Run rust checks/tests outside nix  * add missing components  * Cache rust builds  * Bump holochain deps  * Update rustdocs  * Build cargo deps separately  * refactor flake  * Update maintenance docs  * Add extraPullNames input to cachix actions  * Add nix_build step  * Extend extra-substituters and public-keys in nixConfig  * Fix ci workflow  * Bump client-js version
- Enhance custom template generation (#404) by @c12i in [#404](https://github.com/holochain/scaffolding/pull/404)
  - Feat: Add a new template subcommand to create new custom template bases from existing templates  * Remove call to wrapCustoTemplate in scaffolding nix flake  * Update custom template docs
- Reprompt user on invalid input (#402) by @c12i in [#402](https://github.com/holochain/scaffolding/pull/402)
- Improve `hc-scaffold entry-type` developer experience (#383) by @c12i in [#383](https://github.com/holochain/scaffolding/pull/383)
  - Refactor enum type selection; add option to restart field type selection; refactor FieldType parsing  * Improve EntryTypeReference parsing  * Improve parse_enum parsing logic  * Update field_type parsing logic
- Create links from/to `ExternalHash` (#380) by @c12i in [#380](https://github.com/holochain/scaffolding/pull/380)
  - Add ExternalHash field types and option to target this type when scaffolding a link-type  * ensure external hash is imported in type declarations  * feat: create links from/to AnyLinkableHash  * Add AnyLinkablehHash field type templates and add to reserved words  * fix failed to resolve errors  * improve checking for reserved keywords  * fix invalid link-type delete method in test template  * simplify reserved_words hashsets  * update cli  * Extend reserved keywords to check for javascript keywords  * Update AnyLinkableHash sample value  * Extend reserved words check tests  * fix AnyLinkableHash link-type tests  * Fix AnyLinkableHash link-type tests and remove redundant AND/OR hbs helpers  * update inner_choose_referenceable  * /AnyLinkableHash/ExternalHash  * Update invalid serserved word error message  * Refactor entry/link type utils  * Add some context to the [None] option when scaffolding a link-type  * /AnyLinkableHash/ExternalHash in link-type template  * Fix option placement  * Prevent UI from getting generated where the base type of a link is an ExternalHash  * ExternalHash links can be bidirectional  * Only skip ui if to_referenceable is some and the field_type is of ExternalHash  * Remove unnecessary into call in delete link function  * Fix rustfmt ci failure  * Fix missing conversion  * Fix react link-type template
- Add optional cli flag to skip test code generation (#381) by @c12i in [#381](https://github.com/holochain/scaffolding/pull/381)
- Standardize template styling (#321) by @c12i in [#321](https://github.com/holochain/scaffolding/pull/321)
  - Update collection instructions  * update hbs editor configs  * move generic styles to generic template  * move assets to generic template  * fix misplaced generic styles  * update vue template main component  * remove vue template stylesheet and update index html  * remove reduntant public dir on vue template  * standardize entry-type create component  * standardize entry-type detail component  * standardize entry-for-linke-from component  * templates(vue): standardize edit component  * fix root component  * fix generic styles and vue App.vue component template  * templates(vue): standardize collection component  * templates(react): fix broken css import  * templates(vue): standardize link-type and collection  * templates(vue): update field type components  * templates(vue): remove dependencies  * templates(vue): update templates  * templates(vue): update error handling  * templates(vue): update templates  * templates(svelte): broken path in entrypoint  * templates(svelte): remove redundant templates; import generic styles  * templates(svelte): update entrypoint svelte template  * templates(vue): remove unnecessary imports from vue example template  * templates(vue): update entry-type template  * tempalates(svelte): standardize entry-type templates  * templates(svelte): update templates  * fix clippy warnings  * templates(svelte): refactors  * templates(svelte/vue): update field types  * templates(svelte): fix errors formatting svelte code  * templates(svelte): fix datetime input  * add missing label  * templates(svelte): update example template  * templates(vue/svelte): fix example templates  * templates(svelte): fix field types templates  * templates(lit): standardize templates  * fix vec field types  * templates(lit): fix entry-type file and example template  * detach app styles from generic styles  * templates(lit): fix eslint rules  * templates(lit): fix image imports  * rename record  * templates(lit): use unsafe css  * templates(lit): fix unapplied css  * templates: simplify collection instructions  * rename pnpm-workspace.yaml template  * templates(lit): provide formatting instructions  * templates(lit): fix field type elements  * remove reformatting instructions  * fix vec field type  * remove unnecessary import from example root  * fix select component  * templates(lit): account for cardinality for field types  * update entrypoint  * fix field type events  * templates: remove vec<T> ui  * templates(lit): rename globalStyles to sharedStyles  * fix broken link to css file in example App.tsx  * replace article tags with divs  * remove material ui URL in index.html templates  * refactor shared styles in lit template  * fix holochain logo in assets  * fix svelte templates  * fix missing imports  * fix lit and svelte templates
- Format with rustfmt if available (#354) by @c12i in [#354](https://github.com/holochain/scaffolding/pull/354)
- Add fast track initial text to dna and zome prompts (#353) by @c12i in [#353](https://github.com/holochain/scaffolding/pull/353)
- Programatically format generated svelte and vue markup (#333) by @c12i in [#333](https://github.com/holochain/scaffolding/pull/333)
  - Feat: programatically format generated svelte and vue markup  * refactor format code function  * refactor format_nested and vue example templates  * fix clippy warnings  * return Cow from format_nested  * fix inline jsx/tsx expressions not getting formatted  * update vue template  * fix vue templates
- Make scaffolding cli package manager agnostic (#316) by @c12i in [#316](https://github.com/holochain/scaffolding/pull/316)
  - Add package manager module  * add test cases  * add bun support  * refactor test filetree to resemble actual hApp filetree  * update setup_git_environment  * add package manager command handlebars helper  * add test cases for package_manager_command helper  * use package_manager_command helper  * update ui package.json templates  * replace random port package  * set pnpm install command to recursive  * add corepack_20 package to include required packages  * fix test_run_with_pnpm test  * add bun to packages  * add short version for package_manager flag  * use pnpm on tests  * update hello world example ui script  * update template readme  * revert to npm  * add mising version field in tests/package.json  * update rimraf  * fix hello world template  * remove unnecessary depdencencies from vanilla template  * add missing rimraf dependency  * add missing package_manager field to example hbs context data  * add cli docs  * update lib.rs docs  * fix failing doc tests  * limit package manager option to specific commands that only need it  * update docs
- Replace command with git2 by @c12i
- Add react template (#295) by @c12i in [#295](https://github.com/holochain/scaffolding/pull/295)
  - WIP  * update ui module  * add react to matched ui framework strings  * hbsify templates  * add base react components  * react collection  * react detail component  * react create component  * react edit template  * react entry type for linked from component  * field types  * react link type components  * fix field types  * template swoop; fix issues  * add missing imports, strip out eslint  * fix images  * refactor holochain provider  * fix entry-type templates  * fix link-type templates  * correctly name field type widgets  * add forum example  * fix holochain context  * check for links lengths before setting state  * fix text field  * styling improvments  * extend base styles  * improve example styling  * fix mismatched components  * remove ui readme  * fix templates  * fix imports  * update elements and imports  * type app signal callback  * test with react template in ci  * update styling  * fix global styles  * rename and move client context  * make client context compatible with holo  * fix indentation in templates  * update client context template  * fix entry-types for linked from component props  * update detail components  * add default field types  * fix example app.tsx  * fix vec detail render  * fix vec edit render  * fix timestamp  * fix client context not updating loading state  * fix vec input  * fix vec input setters in edit components  * Update templates/react/field-types/u32/NumberInput/edit/render.hbs
- Add long disable fast track option to hc scaffold web-app (#293) by @c12i in [#293](https://github.com/holochain/scaffolding/pull/293)
- Add headless template (#267) by @c12i in [#267](https://github.com/holochain/scaffolding/pull/267)
  - Feat: add initial headless template  * include headless template in ui framework choices  * add web-app instructions and prefer explicit hc commands in all package.json templates  * rename instructions template  * fix indentation in instructions  * update instructions  * remove gitkeep  * refactor UiFramework struct  * refactor tempalte config access  * remove assertion  * revert renaming  * revert integrity zome name  * choose non vanilla framework for non hello world examples  * simplify ui framework prompts; update instructions  * update instructions  * Update scripts and instructions  * color code ui frameworks  * templatify web-happ manifest  * fix bug writing invalid template config  * remove unnecessary print statement  * remove unnecessary return statement  * update ui framework try from filetree implementation  * refactor: refactor reserved words check  * update ansi colors for lit and svelte  * Add build happ instruction
- Add fast track flag to hc-scaffold web-app (#252) by @c12i in [#252](https://github.com/holochain/scaffolding/pull/252)
  - Feat: add fast forward flag to hc-scaffold web-app  * feat refactor cli code  * revert refactor to app file tree get or choose method  * remove unused import  * fix output formatting  * feat: improve initial instructions on scaffolding a web-app  * fix clippy errors  * italicize named values  * fix clippy warning  * rename fast_forward to fast_track  * remove unused variable  * Update fast track flag, and have it true by default  * Fix wrongly written dna manifest  * Add missing dependency to coordinator zome  * Add FsBuildError variant  * Remove unused dependency  * fix clippy warnings
- Write template config for examples (#269) by @c12i in [#269](https://github.com/holochain/scaffolding/pull/269)
  - Feat: write template config for examples  * fix instruction formatting
- Colorize error messages to red (#256) by @c12i in [#256](https://github.com/holochain/scaffolding/pull/256)
  - Feat: colorize error messages to red  * prefer eprintln
- Add option to set agent count via env var by @c12i
- Add file_exists handlebars helper by @c12i
- Skip ui for link and entry-type by @c12i
- Initial skip-ui implementation on collections by @c12i

### Bug Fixes

- *(flake)* Workaround unexpected flake behavior by @steveej
- *(nix)* Pkgs -> packages by @steveej
- *(svelte)* Ensure client is loaded before use (#465) by @c12i in [#465](https://github.com/holochain/scaffolding/pull/465)
  - Ensure svelte client is loaded
  - Fix left over changes from merge
  - Remove unused imports in context
  - Wrap app in client provider
- *(template-svelte)* Remove duplicated imports by @c12i
- *(template-vue)* Add missing comma by @c12i
- *(templates)* Add missing imports and types in test templates by @c12i
- *(templates-vue)* Add missing key props, use indexes by @c12i
- Repair logic to throw meaningful error if scaffolding with nix setup is attempted in an existing git repo (#516) by @matthme in [#516](https://github.com/holochain/scaffolding/pull/516)
- Set working directory before running cargo fmt (#514) by @matthme in [#514](https://github.com/holochain/scaffolding/pull/514)
- Wrongly used v-if directive (#457) by @c12i in [#457](https://github.com/holochain/scaffolding/pull/457)
  - Fix incorrect usage of v-if directive  * Fix vue-ts type error on current record prop  * Check both editing and record
- Gracefully handle number prefixed names (#439) by @c12i in [#439](https://github.com/holochain/scaffolding/pull/439)
- Bump hc-spin for 0.5 (#420) by @c12i in [#420](https://github.com/holochain/scaffolding/pull/420)
  - Bump hc-spin for 0.5  * Prepare a weekly release  * Bump versions
- Bump `client-js` and update usage (#419) by @c12i in [#419](https://github.com/holochain/scaffolding/pull/419)
  - Remove cap_secret param from callZome api call  * Update client version and usage; also fix console errors in vue template  * Pin typescript version in vue template to maintain compatibility with vue-tsc
- Skip writing scaffold config for nixified custom templates (#415) by @c12i in [#415](https://github.com/holochain/scaffolding/pull/415)
  - Add skip_config_check global flag to scaffolding  * Prefer not writing the scaffold config for nixified custom templates  * Fix rustfmt warning
- Fix misplaced doc comments (#411) by @c12i in [#411](https://github.com/holochain/scaffolding/pull/411)
  - Fix misplaced doc comments  * Slightly refactor link_type integrity codegen
- Refactor/custom templates (#397) by @c12i in [#397](https://github.com/holochain/scaffolding/pull/397)
  - Fix custom template flake  * Refactor custom templates
- Optimize nix flake (#384) by @c12i in [#384](https://github.com/holochain/scaffolding/pull/384)
  - Optimize nix flake  * Supress clippy warnings
- Preserve line comments on creating/ modifying rust code (#382) by @c12i in [#382](https://github.com/holochain/scaffolding/pull/382)
  - Refactor un_parse_pretty function  * Move important scaffold functions to the top of the module for easy accesss  * fix improper use of bool.then_some  * ensure line comments are preserved  * derive a convert rust line comments to doc comments util function and apply to map_rust_files function  * ensure initially generated comments are correctly formatted  * remove unnecessary inline attribute on unparse_pretty  * Replace once_cell::Lazy with standard lib LazyLock
- Parametrize package manager related nixpkgs on scaffolded flakes (#373) by @c12i in [#373](https://github.com/holochain/scaffolding/pull/373)
- Add custom PS1 prompt nix shell hook (#375) by @c12i in [#375](https://github.com/holochain/scaffolding/pull/375)
- Invalid variable name in vanilla template (#374) by @c12i in [#374](https://github.com/holochain/scaffolding/pull/374)
- Invalid options when scaffolding an example without cli options (#369) by @c12i in [#369](https://github.com/holochain/scaffolding/pull/369)
  - Fix invalid options when scaffolding an example without cli options  * largely refactor the ui module and transform it into a template-type module  * fix: retrun early with custom template name  * limit forum template to non-vanilla templates  * Fix unhandled error in TemplateType Serialize impl
- Update lit templates (#366) by @c12i in [#366](https://github.com/holochain/scaffolding/pull/366)
- Pnpm-workspace .hbs file generated on scaffolding a web-app/example using non-pnpm package manager (#357) by @c12i in [#357](https://github.com/holochain/scaffolding/pull/357)
- Use pnpm workspace yaml file with pnpm happs (#340) by @c12i in [#340](https://github.com/holochain/scaffolding/pull/340)
- Missing dependency with lit template using `pnpm` (#334) by @c12i in [#334](https://github.com/holochain/scaffolding/pull/334)
  - Fix: add missing tslib dependency on lit template  * use pnpm in ci  * use parameterized package manager in ci wf template  * fix misplaced fix
- Handle hdk_extern annotated functions without inputs (#331) by @c12i in [#331](https://github.com/holochain/scaffolding/pull/331)
- Improve ts codegen and formatting for ts/tsx files (#311) by @c12i in [#311](https://github.com/holochain/scaffolding/pull/311)
  - Refactor: replace tempalte derived types.ts files with rust-geneted ones  * fix typo  * add programatic formatting  * fix incorrectly named variable in tests  * ignore formatting EntryTypes  * fix invalid enum types  * fix: improve code formatting in ts/tsx files  * fix clippy warning  * fix failing codgegen tests  * rename tests module
- Include react in testable templates; bump versions (#310) by @c12i in [#310](https://github.com/holochain/scaffolding/pull/310)
  - Fix: add react to testable templates  * bump crate version; nix flake update
- Holochain client error handling (#298) by @c12i in [#298](https://github.com/holochain/scaffolding/pull/298)
  - Fix: holochain client error handling  * fix: typescript failures in lit template
- Holo web sdk version not part of ScaffoldWebAppData (#302) by @c12i in [#302](https://github.com/holochain/scaffolding/pull/302)
  - Fix: Add missing web-sdk-version to template data  * rename holo web sdk template field
- Poorly formatted rust code (#288) by @c12i in [#288](https://github.com/holochain/scaffolding/pull/288)
  - Fix: poorly formatted code in coordinator zome  * document add_newlines  * clearn up coordinators module  * clean up integrity module  * refactor utils  * fix add_newlines  * update integrity comments  * fix map file calls  * fix add_newlines  * make add_newlines public/private function agnostic  * refactor link-type coordinator  * fix  remove link handler  * fix collection coordinator  * clean up entry-type coordinator  * fix clippy warnings  * refactor link-type coordinator  * fix bug scaffolding entry-type with linked_from  * fix target_hash_variable  * remove unnecessary format usage  * remove unnecessary clone  * fix validate_delete_result  * move comment  * fix entry from rcord error message  * rename unparse function to unparse_pretty;
- Add missing headless field type samples (#292) by @c12i in [#292](https://github.com/holochain/scaffolding/pull/292)
  - Add missing headless field types  * add missing package command from svelte scripts
- Bug while scaffolding collection by author (#290) by @c12i in [#290](https://github.com/holochain/scaffolding/pull/290)
  - Fix: bug while scaffolding collection by author  * fix initial insert statement
- Fix vue example ui server not starting (#284) by @c12i in [#284](https://github.com/holochain/scaffolding/pull/284)
- Prevent scaffolding dead code (#280) by @c12i in [#280](https://github.com/holochain/scaffolding/pull/280)
  - Fix: prevent scaffolding dead code  * fix clippy warnings  * improve readability of stringified rust code  * update coordinator functions  * refactor boolean checks
- Prevent unaffected rust files from getting formatted (#281) by @c12i in [#281](https://github.com/holochain/scaffolding/pull/281)
  - Fix: preserve doc comments  * fix: unmodified files getting reformatted by unparse  * refactor map_all_files_rec function  * revert unparse function change
- Invalid entry type fields in vue template (#279) by @c12i in [#279](https://github.com/holochain/scaffolding/pull/279)
- Limit examples to non vanilla ui frameworks (#272) by @c12i in [#272](https://github.com/holochain/scaffolding/pull/272)
- Bug when running hello world example (#271) by @c12i in [#271](https://github.com/holochain/scaffolding/pull/271)
  - Fix: fix hello world example  * bump version
- Infer vanilla template for hello world example (#262) by @c12i in [#262](https://github.com/holochain/scaffolding/pull/262)
  - Fix: error out on incompatible template + example  * fix: infer vanilla template for hello world example  * fix clippy warnings
- Ensure is holo env var is passed at build time (#251) by @c12i in [#251](https://github.com/holochain/scaffolding/pull/251)
  - Fix: Ensure is holo env var is passed at build time  * fix: only pass the env var is holo is enabled  * have separate package scripts  * Remove root package holo script  * fix: fix json parse error
- Limit template matching to inbuilt templates (#248) by @c12i in [#248](https://github.com/holochain/scaffolding/pull/248)
- Prefer dynamic ports for launching ui by @c12i
- Make lit template reactive by @c12i
- Make vue template reactive by @c12i
- Make svelte template reactive by @c12i
- Add reserved word check for entry-type field name by @c12i
- Address breaking changes on get_link_details by @c12i
- Remove target dir from ui gitignore by @c12i
- Fix folders not getting correctly gitignored by @c12i
- Fix lit template values by @c12i
- Fix proc macro indentation by @c12i
- Prefer std lib tempdir by @c12i
- Make no_ui flag visible for entry-type by @c12i
- Remove unwrap by @c12i
- Extend example template data struct by @c12i
- Update generated README to use newer nix shell command by @c12i
- Fix typo in link-type flag by @c12i
- Fix typo in example name by @c12i
- Fix ci errors while calling command and args in nix shell by @c12i
- Prefer exported env variables by @c12i
- Use matrix by @c12i
- Consider browsers that do not support top level await by @c12i
- Prevent input of multiple of the same enum variants by @c12i
- Fix conflicting enum name nad entry type name producing broken code by @c12i
- Update holochain version by @c12i
- Fix collection type by author parse error by @c12i
- File in ui template options by @c12i
- 'edition' not supported in workspace manifest, use resolver = '2' instead by @mattyg
- Specify rust edition 2021 (which includes resolver '2') to ensure sweettests can be included in zome builds by @mattyg
- Bump version and add Cargo.lock by @steveej

### Miscellaneous Tasks

- *(templates)* Update holochain client usage in templates by @c12i
- Bump to 0.600.0-dev.0 by @ThetaSinner in [#519](https://github.com/holochain/scaffolding/pull/519)
- Remove redundant words by @hustrust in [#510](https://github.com/holochain/scaffolding/pull/510)
- Update to Holochain 0.5.5 by @ThetaSinner
- Update to Holochain 0.5.3 by @ThetaSinner in [#502](https://github.com/holochain/scaffolding/pull/502)
- Update project Rust version to 1.87 by @ThetaSinner in [#501](https://github.com/holochain/scaffolding/pull/501)
- Update to nixos-25.05 and update flake inputs by @ThetaSinner
- Update project dependencies by @ThetaSinner
- Upgrade Nix and action versions used in CI by @ThetaSinner
- Add clippy checks to test script and fix existing warnings (#491) by @c12i in [#491](https://github.com/holochain/scaffolding/pull/491)
- Bump vite and other dev deps (#490) by @c12i in [#490](https://github.com/holochain/scaffolding/pull/490)
  - Chore: Bump vite and dev dependencies
  - Fix: Add missing @types/node to react ui
  - Chore: Clean ups on vanilla example
  - Chore: Bump test workspace dependencies
  - Chore: Fix vite vs vitest
  - 
- Bump react and related dependencies (#489) by @c12i in [#489](https://github.com/holochain/scaffolding/pull/489)
- Bump to 0.5.1 (#488) by @c12i in [#488](https://github.com/holochain/scaffolding/pull/488)
  - Chore: Bump versions
  - Chore: nix flake update
  - Fix: Remove unused fields
  - Chore: nix flake update and cargo update
  - Fix: remove broken fields
  - Ci: update rust-toolchain
  - Chore: add rust-toolchain.toml
  - Fix: Remove origin time field
  - Chore: nix flake update
  - Chore: Update client version
  - Chore: Replace run-local-services with kitsune2-bootstrap-srv
  - Chore: nix flake update
  - Chore: bump versions to rc
  - Chore: update nix flake
  - Chore: Bump version
  - Chore: bump tryorama
  - Fix: add js-yaml
  - Chore: bump hc-spin version
  - Chore: Bum tryorama version; remove js-yaml dependency
  - Chore: Remove targets from rust-toolchain
  - Chore: Use old versioning strategy
  - Chore: Bump versions
  - Chore: Bump versions
  - Chore: Bump versions
  - Switch playground source to nix
  - Chore: Nix flake update
  - Chore: Update holonix branch
  - Chore: Remove quantum time modifier
  - Chore: Bump hc-spin
  - Chore: Bump to 0.5.1
  - Include PR https://github.com/holochain/scaffolding/pull/467 (#494)
  - I don't know why, these changes made in https://github.com/holochain/scaffolding/pull/467 are not in this branch anymore.
  - Chore: Update tryorama version to released
  - Chore: Remove unused SIGNAL_PORT
  - 
- Bump version (#483) by @c12i in [#483](https://github.com/holochain/scaffolding/pull/483)
  - Bump version
  - Nix flake update
- Migrate vue templates to composition api (#473) by @c12i in [#473](https://github.com/holochain/scaffolding/pull/473)
  - Fix client loading in vue
  - Add missing imports
  - Add holo features
  - Fix signal usage
  - Update templates/ui-frameworks/vue/collection/ui/src/{{dna_role_name}}/{{coordinator_zome_manifest.name}}/{{pascal_case collection_name}}.vue.hbs
- Bump weekly to holochain 0.5.0 dev.20 (#455) by @c12i in [#455](https://github.com/holochain/scaffolding/pull/455)
  - Bump versions
  - Nix flake update
  - Bump holochain_types
  - Pin holo-server-bin version to most recent working version
  - Unpin holo-dev-server-bin
  - Update ci workflow
  - Bump to 0.5.0-dev.17
  - Nix flake update
  - Ci: Add `ci_pass` job (#454)
  - Ci: Add `ci_pass` job  * ci: Rename develop to main
  - Fix lit template ts error (#448)
  - Fix: wrongly used v-if directive (#457)
  - Fix incorrect usage of v-if directive  * Fix vue-ts type error on current record prop  * Check both editing and record
  - Bump to dev.18
  - Nix flake update
  - Remove test retries
  - Bump versions
  - Nix flake update
  - Fix signal usage
  - Bump tryorama
  - Bump dep versions
  - Nix flake update
  - Update enum usage in tests
  - Fix slow tests
  - Update app bundle sources
  - 
- Bump vitest and update test config (#463) by @c12i in [#463](https://github.com/holochain/scaffolding/pull/463)
  - Update vitest and add test retries  * Run tests in single thread  * Increase test timeout  * Update templates/generic/web-app/tests/vitest.config.ts.hbs
- Bump weekly `0.500.0 dev.5` (#444) by @c12i in [#444](https://github.com/holochain/scaffolding/pull/444)
  - Bump dependencies  * Bump version  * Nix flake update  * Bump to holochain dev.13
- Release weekly 0.500.0 dev.2 (#416) by @c12i in [#416](https://github.com/holochain/scaffolding/pull/416)
- Update ui dependencies (#407) by @c12i in [#407](https://github.com/holochain/scaffolding/pull/407)
  - Bump holochain playground  * Update ui core dependencies  * Update lit dependencies  * Fix cli output lines  * Fix broken css import in lit template  * Revert custom-template changes  * Fix vue templates  * Increase testTimeout  * Remove eslint from lit template  * Fix lit CI failure  * Add missing shared styles
- Bump weekly (#409) by @c12i in [#409](https://github.com/holochain/scaffolding/pull/409)
  - Bump versions  * Nix flake  update
- Update holochain dependencies (#398) by @c12i in [#398](https://github.com/holochain/scaffolding/pull/398)
  - Update dependencies  * Remove unused dependency
- Update 0.4 flake inputs (#372) by @c12i in [#372](https://github.com/holochain/scaffolding/pull/372)
  - Chore: update 0.4 flake inputs  * update nix flake
- Bump weekly (#359) by @c12i in [#359](https://github.com/holochain/scaffolding/pull/359)
  - Chore: bump weekly  * chore: bump tryorama , holochain client and hc-spin versions  * nix flake update  * fix dprint_plugin_typescript breaking change
- Switch to `github:holochain/holonix` (#362) by @c12i in [#362](https://github.com/holochain/scaffolding/pull/362)
  - Update root nix flake  * update webhapp nix flake  * add ci devShell  * update ci and default devshell, fix craneLib file filtering; update ci/cd workflow  * remove local extend-space action  * remove nix experimental features option in ci  * remove unnecessary svelte checks; update custom template flake  * update flake.nix;  * update happ flakes  * use yarn-berry over yarn nixpkg  * update lock files
- Bump versions by @c12i
- Bump version by @c12i
- Bump holochain version (#341) by @c12i in [#341](https://github.com/holochain/scaffolding/pull/341)
  - Bump holochain version  * update client-js version  * nix flake update  * fix clippy warnings
- Bump weekly by @c12i
- Bump client versions (#336) by @c12i in [#336](https://github.com/holochain/scaffolding/pull/336)
- Dependency upgrades (#320) by @c12i in [#320](https://github.com/holochain/scaffolding/pull/320)
  - Chore: minor dependency upgrades  * fix formatting  * feat: programatically format generated svelte and vue markup (#333)  * feat: programatically format generated svelte and vue markup  * refactor format code function  * refactor format_nested and vue example templates  * fix clippy warnings  * return Cow from format_nested  * fix inline jsx/tsx expressions not getting formatted  * update vue template  * fix vue templates  * fix: Missing dependency with lit template using `pnpm` (#334)  * fix: add missing tslib dependency on lit template  * use pnpm in ci  * use parameterized package manager in ci wf template  * fix misplaced fix  * keep bundled_dnas_paths function private
- Bump holochain 0.4.0 dev.12 (#327) by @c12i in [#327](https://github.com/holochain/scaffolding/pull/327)
  - Chore: bump holochain version 0.4.0-dev.12  * nix flake update  * handle errors in empty_dna_manifest function  * fix DnaManifest builder usage
- Bump holochain version 0.4.0-dev.10 (#318) by @c12i in [#318](https://github.com/holochain/scaffolding/pull/318)
- Update docs by @c12i
- Bump version by @c12i
- Bump versions (#312) by @c12i in [#312](https://github.com/holochain/scaffolding/pull/312)
  - Chore: bump versions  * bump hc-spin
- Bump weekly (#304) by @c12i in [#304](https://github.com/holochain/scaffolding/pull/304)
  - Chore: bump weekly  * chore: bump to 0.4.0-dev.7  * nix flake update  * chore: bump holochain playground cli
- Update manual testing docs (#285) by @c12i in [#285](https://github.com/holochain/scaffolding/pull/285)
  - Chore: update manual testing docs  * update override
- Update web-app instructions for headless template by @c12i
- Bump weekly (#276) by @c12i in [#276](https://github.com/holochain/scaffolding/pull/276)
  - Chore: bump weekly  * remove todo comment
- Remove unused dependencies (#277) by @c12i in [#277](https://github.com/holochain/scaffolding/pull/277)
  - Chore: remove unused dependencies  * chore: bump version
- Bump holochain version `0.3.0-beta-dev.48` (#270) by @c12i in [#270](https://github.com/holochain/scaffolding/pull/270)
  - Bump holochain dependencies  * update FlatOp::RegisterUpdate validation arm  * update initial FlatOp::RegisterDelete  * update FlatOp::RegisterDelete  * update manifest and lock files  * fix unreachable code in update validation match arm  * update FlatOp::RegisterDelete code  * bump holochain dependencies  * nix flake update  * bump hc spin  * bump client-js version  * update flake lock  * remove unused imports  * bump client-js version  * bump version  * update flake lock  * bump client-js version  * bump tryorama  * add disable fast forward flag to test script  * update app websocket usage  * bump hc spin  * replace version functions with constants  * parameterize holochain playground cli version  * update holochain playground cli version  * add doc comments to version consts  * add disable fast forward flag to holo-integration test script  * improve register delete arm body check, prefer parse quote where there is no interpolation  * update register delete  * make conditional checks explicit/ declarative  * fix holo integration script runtime  * fix comparisons; revert validation  * refactor add_entry_type_to_validation_arms function  * handle potential validation errors  * improve validation error messages  * further improvements to code readability  * fix clippy warnings  * reduce nested code  * prefer parse_quote for non interpolated syn items  * refactor render_entry_definition_file  * refactor render_entry_definition_struct  * fix indentation  * prefer parse_quote and avoid unwrapping in map closure  * bump tryorama  * use consistent versioning  * prefer Display trait implmenentation to ToString  * code refactoring n uis module  * fix formatting
- Bump holochain client js, tryorama and hc spin versions (0.3) (#264) by @c12i in [#264](https://github.com/holochain/scaffolding/pull/264)
  - Chore: bump holochain client js and tryorama versions  * chore: bump hc-spin  * chore: bump crate version  * chore: update nix flake  * chore: pin hc-spin version
- Bump hc-spin version (#263) by @c12i in [#263](https://github.com/holochain/scaffolding/pull/263)
- Update templates (#259) by @c12i in [#259](https://github.com/holochain/scaffolding/pull/259)
  - Fix: remove instances of file_exists helper from custom template  * fix: remove redundant package:holo script from vue template
- Bump holochain crate versions (#250) by @c12i in [#250](https://github.com/holochain/scaffolding/pull/250)
  - Bump holochain crate versions  * Bump holochain client version  * chore: Update nix flake  * chore: bump holochain client js version  * chore: bump tryorama version  * chore: update nix flake  * chore: update hdk_extern usage  * chore: update hc-spin version  * chore: update holochain core crates dependency versions  * bump version  * update cargo lock file
- Improve no_ui flag documentation and warning (#257) by @c12i in [#257](https://github.com/holochain/scaffolding/pull/257)
- Bump holochain crate versions by @c12i
- Bump holochain crate versions by @c12i
- Simplify test script and update contributing guide by @c12i
- Bump ci action versions by @c12i
- Update nix flake again by @c12i
- Update nix flake by @c12i
- Bump holochain dependency versions by @c12i
- Bump tryorama version by @c12i
- Update holochain client version by @c12i
- Nix flake update by @c12i
- Bump crate version by @c12i
- Bump holochain versions by @c12i
- Bump msgpack dependency by @c12i
- Group global gitignore by @c12i
- Extend ignored files/folders by @c12i
- Bump crate version by @c12i
- Bump hc_spin_version by @c12i
- Add client version to example package.json by @c12i
- Fix formatting in file exists by @c12i
- Add test cases by @c12i
- Update no_ui doc on entry-type by @c12i
- Remove comments by @c12i
- Document no_ui flag by @c12i
- Rename skip_ui to no_ui by @c12i
- Use lts node for happs by @c12i
- Update holochain client version by @c12i
- Print version by @c12i
- Reformat script and rename wf step by @c12i
- Bump crate version by @c12i
- Use serde 1.0 by @c12i
- Update flake.nix by @c12i
- Add valid URL string in lit example by @mattyg
- Use correct name in flake.nix by @c12i
- Use LTS node version by @c12i
- Prefer single import by @c12i
- Use latest version of serde by @c12i
- Update flake.nix by @c12i
- Update hdi and hdk versions in example by @c12i
- Update tryorama version by @c12i
- Update get_links calls to use GetLinksInputBuilder by @c12i
- Update all references of hdk_entry_defs to hdk_entry_types by @c12i
- Update tryorama and holochain client versions by @c12i
- Update holochain nix version by @c12i
- Fix spelling 'bidireccional' -> 'bidirectional' by @mattyg
- Remove unnecessary changes by @c12i

### Build System

- Bump nixpkgs for crane support (#479) by @mattyg in [#479](https://github.com/holochain/scaffolding/pull/479)
- Cargo lock by @mattyg

### CI

- Update release actions by @ThetaSinner in [#513](https://github.com/holochain/scaffolding/pull/513)
- Update action versions and install newer Nix by @ThetaSinner in [#506](https://github.com/holochain/scaffolding/pull/506)
- Add CI workflows for automated releases by @ThetaSinner
- Add `ci_pass` job (#454) by @ThetaSinner in [#454](https://github.com/holochain/scaffolding/pull/454)
  - Ci: Add `ci_pass` job  * ci: Rename develop to main
- Address clippy and fmt errors and add ci checks (#245) by @c12i in [#245](https://github.com/holochain/scaffolding/pull/245)
  - Run cargo fmt  * Fix clippy errors  * refactor: prefer slices  * feat: add cargo clippy and fmt ci checks  * fix initial lib.rs  * fix ci workflow file  * Merge remote-tracking branch 'holochain/develop' into add-clippy-and-fmt-checks-to-ci-workflow  * fix more clippy errors  * fix formatting  * fix clippy errors in entry_type module  * fix obfuscated_if_else  * fix fmt  * add todo comments on clippy::too_many_arguments  * merge ci workflow  * fix typo on step  * prefer borrowed path type  * fix: address lint warnings in generated code  * consistent string expressions  * fix ci failure on integrity zome  * fix bug in coordinator codegen  * fix boolean checks
- Build nix package by @steveej
- Make tests work with `nix-shell --pure` by @steveej
- Don't attempt to install yarn (globally) by @steveej
- Use ubuntu-latest and don't setup node via ubuntu by @steveej
- Use cachix-action and split steps by @steveej

### Refactor

- Cli module (#317) by @c12i in [#317](https://github.com/holochain/scaffolding/pull/317)
  - Refactor: cli module to command submodules  * add missing doc strings  * fix clippy warnings  * update cli/zome.rs  * inline some functions  * set package manager explicitly on holo integration scripts  * limit inlined functions  * implement FromStr for FieldDefinition  * refactor scaffold/app/cargo.rs  * update entry_type/crud.rs  * fix clippy warnings  * add removed fast track config  * fix clippy warnings  * prefer if else to pattern matching on boolean expressions where possible  * move warning missage to scaffold_entry_type  * remove template arg
- Refactor templates module and handle unhandled errors (#185) by @c12i in [#185](https://github.com/holochain/scaffolding/pull/185)
  - Refactor: Refactor templates module and handle unhandled errors  * fix: fix OsStr to str conversion with try_into  * chore: fix typo  * chore: rename MiscError variant display message  * Merge remote-tracking branch 'holochain/develop' into refactor-tempaltes-module  * set regexes as static variables  * fix clippy warnings  * fix clippy warnings
- Clean up duplicated templates (#313) by @c12i in [#313](https://github.com/holochain/scaffolding/pull/313)
  - Derive common templates  * derive a scaffold config module and move config related code to it  * remove field types from generic templates  * delete repeated templates  * fix clippy warnings  * further trim down repeated templates
- Replace 'pause' with 'dhtSync' in tests by @mattyg

### Documentation

- Update manual testing docs (#371) by @c12i in [#371](https://github.com/holochain/scaffolding/pull/371)
  - Docs: manual testing docs  * update docs

### WIP

- Use flake.nix by @steveej

### Bug

- Fix invalid class names in react template (#370) by @c12i in [#370](https://github.com/holochain/scaffolding/pull/370)
- Fix hc scaffold link type issues (#368) by @c12i in [#368](https://github.com/holochain/scaffolding/pull/368)
  - Fix hardcoded case in check_case function  * fix app crash on scaffolding link_type with target set to [None]

### Default.nix

- Use gitignoreSource (not only) to assist debugging by @steveej

### Enhancement

- Improve ci runtime by @c12i
- Use parse(from_os_str) for PathBuf args to handle non-UTF-8 file paths on windows by @c12i

### Generators

- Add tslib dependency by @steveej

### Revert

- Remove file_exists handlebars helper (#253) by @c12i in [#253](https://github.com/holochain/scaffolding/pull/253)
  - Revert: Remove file exists helper usage in templates  * revert: Remove file exists handlerbars helper  * update no_ui flag docs  * feat: print warning on --no-ui usage
- Revert tryorama version by @c12i

### Update

- *(cli)* Bump nixos to 22.11 by @steveej

### First-time Contributors

- @mattyg made their first contribution in [#522](https://github.com/holochain/scaffolding/pull/522)
- @ThetaSinner made their first contribution in [#519](https://github.com/holochain/scaffolding/pull/519)
- @matthme made their first contribution in [#516](https://github.com/holochain/scaffolding/pull/516)
- @hustrust made their first contribution in [#510](https://github.com/holochain/scaffolding/pull/510)
- @c12i made their first contribution in [#491](https://github.com/holochain/scaffolding/pull/491)
- @pdaoust made their first contribution in [#427](https://github.com/holochain/scaffolding/pull/427)
- @emhoracek made their first contribution in [#138](https://github.com/holochain/scaffolding/pull/138)
- @r-vdp made their first contribution
- @guillemcordoba made their first contribution
- @joshuavial made their first contribution
- @robbiecarlton made their first contribution in [#132](https://github.com/holochain/scaffolding/pull/132)
- @ made their first contribution
- @steveej made their first contribution
- @zippy made their first contribution
- @harlantwood made their first contribution
# Changelog

All notable changes to this project will be documented in this file. 

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[[0.600.0-dev.0](https://github.com/holochain/scaffolding/compare/v0.500.0...v0.600.0-dev.0)\] - 2025-10-16

### Features

- Remove UI references to the WebSDK by @ThetaSinner in [#512](https://github.com/holochain/scaffolding/pull/512)
- Remove support for Holo enabled scaffolding by @ThetaSinner
- Upgrade to 0.6 by @ThetaSinner

### Bug Fixes

- Repair logic to throw meaningful error if scaffolding with nix setup is attempted in an existing git repo (#516) by @matthme in [#516](https://github.com/holochain/scaffolding/pull/516)
- Set working directory before running cargo fmt (#514) by @matthme in [#514](https://github.com/holochain/scaffolding/pull/514)

### CI

- Update release actions by @ThetaSinner in [#513](https://github.com/holochain/scaffolding/pull/513)

### Miscellaneous Tasks

- Remove redundant words by @hustrust in [#510](https://github.com/holochain/scaffolding/pull/510)

<!-- generated by git-cliff -->
