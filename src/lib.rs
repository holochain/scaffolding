//! # Using the scaffolding CLI tool:
//!
//! After having installed it, these are the commands that are available:
//!
//! ```bash
//! // Scaffold an example Holochain app
//! hc-scaffold example
//! ```
//!
//! Or if you want to scaffold your own custom app, for example a to-do app:
//!
//! ```bash
//! // Scaffold an empty web-app, using a built-in template
//! hc-scaffold web-app todos
//!
//! cd todos
//!
//! // Scaffold a dna inside the newly scaffolded app
//! hc-scaffold dna todos
//!
//! // Scaffold a zome inside the newly scaffolded dna
//! hc-scaffold zome todos
//!
//! // Scaffold an entry-type inside the newly scaffolded zome
//! hc-scaffold entry-type todo
//!
//! // Scaffold a collection for the newly scaffolded entry-type
//! hc-scaffold collection global all_todos
//!
//! // Scaffold a new link-type
//! hc-scaffold link-type
//!
//! // Will show all the commands that are available
//! hc-scaffold --help
//! ```
//!
//! # Custom Templates
//!
//! The scaffolding tool comes with 4 built-in templates:
//!
//! - Vue (with TypeScript)
//! - Svelte (with TypeScript)
//! - Lit (with TypeScript)
//! - Vanilla
//!
//! These templates provide most of the skeleton you need to start your own holochain app.
//!
//! But! They are not complete, nor provide good design from the UI/UX perspective. They are trying to be unopinionated in that regard, so that you as the developer can apply your own style of building frontend apps.
//!
//! To allow for more flexibility, the scaffolding tool can be extended and customized using custom templates. This would allow you to create a "React" template, or a "Vue + tailwind" template, or whatever style of frontend code and packaging you want for your app.
//!
//! ## Using custom templates
//!
//! All `hc scaffold` commands accept an optional `--template` argument. This argument can be:
//! - Either one of the built-in templates:
//!   - "vue"
//!   - "svelte"
//!   - "lit"
//!   - "vanilla"
//! - Or a path to a custom template.
//!   - E.g `hc-scaffold --template ./path/to/custom/template/folder web-app`
//!
//! If you know of some already existing custom template, look first in the documentation of that template for instructions on how to use it, in case the template offers a nix wrapper command, which is much easier to use.
//!
//! Otherwise, you can just clone it and use it like this:  
//!
//! `hc scaffold --template ./path/to/custom/template web-app forum`
//!
//! Notice that you will need to pass the `--template` argument in every command.
//!
//! ## How to create a custom template
//!
//! Creating and maintaining your own template can be challenging at first, so look for existing templates that you can reuse before diving in to create your own.
//!
//! The best way to start creating a custom template is to go from one of the built-in ones, and modify it.
//!
//! To create a custom template, execute these steps:
//!
//! 1. Run this command:
//! `nix flake init -t github:holochain/scaffolding`
//! 2. A new dir `custom-template` will be created in the current directory. Check this new folder in a version control system like git.
//! 3. Replace all instances of `<TODO:REPLACE_ME_WITH_THE_APPROPRIATE_GIT_URL>` in its `README.md` file with the appropriate git URL (eg. "github:holochain-open-dev/templates").
//! 4. Replace all instances of `<TODO:REPLACE_ME_WITH_THE_APPROPRIATE_GIT_URL>` in its `template/web-app/flake.nix.hbs` file with the appropriate git URL (eg. "github:holochain-open-dev/templates").
//!
//! That's it! At this point you will have a correctly functioning custom template repository with tests, a `README.md` documenting how to use it, and a `template` folder. That's where your custom template lives.
//!
//! Templates have this directory structure:
//!
//! coordinator-zome/
//! dna/
//! entry-type/
//! example/
//! field-types/
//! collection/
//! integrity-zome/
//! link-type/
//! web-app/
//!
//! Each folder corresponds to the templates that are created when running a specific command. Here are the steps executed:
//!
//! 1. The user executes a scaffolding command, like `hc scaffold web-app`.
//!    - Optionally, they may pass a `--template` argument, specifying the template name or local path to use.
//!    - The `--template` value is saved in the root `package.json` file's `hcScaffold` key for future reference and to prevent the use of different templates in the same project.
//! 2. The scaffolding tool prompts the user to input all necessary information for the command.
//! 3. **Backend** code is automatically generated, independent of custom templates.
//! 4. If a `--template` argument is provided:
//!    - A check is performed to ensure alignment with the originally scaffolded hApp. If successful, it uses the provided template.
//! 5. For the selected template, the tool searches for a corresponding folder inside it based on the command.
//!    - For example, `hc scaffold web-app` searches for a folder named `web-app`.
//! 6. If found, it copies the directory structure within that folder, selecting files with the `.hbs` extension.
//! 7. It renders the contents of each file using appropriate data from the command.
//!    - For instance, in `hc scaffold web-app`, one context field is `app_name`, representing the user-input app name.
//! 8. Finally, it merges the resulting directory structure with the existing repository. If a file already exists, its contents are overwritten.
//! You can take a look at [Writing templates](#writing-templates) to learn how to write your own templates.
//!
//! This is the list of commands and the templates they use:
//!
//! - `web-app`: uses the `web-app` folder. [Available data](`crate::templates::web_app::ScaffoldWebAppData`).
//! - `dna`: uses the `dna` folder. [Available data](`crate::templates::dna::ScaffoldDnaData`).
//! - `zome`: uses the `coordinator-zome` folder if scaffolding a coordinator zome, and the `integrity-zome` folder if scaffolding an integrity zome. [Available data](`crate::templates::coordinator::ScaffoldCoordinatorZomeData`).
//! - `entry-type`: uses the `entry-type` folder. [Available data](`crate::templates::entry_type::ScaffoldEntryTypeData`).
//! - `link-type`: uses the `link-type` folder. [Available data](`crate::templates::link_type::ScaffoldLinkTypeData`).
//! - `collection`: uses the `collection` folder. [Available data](`crate::templates::collection::ScaffoldCollectionData`).
//! - `example`: uses the `example` folder. [Available data](`crate::templates::example::ScaffoldExampleData`).
//!
//! ### Field types
//!
//! The `field-types` folder is special. It has the following directory structure:
//!
//! ActionHash/
//!   type.hbs
//! AgentPubKey/
//!   type.hbs
//! bool/
//!   Checkbox/
//!     detail/
//!       render.hbs
//!     edit/
//!       imports.hbs
//!       render.hbs
//!   type.hbs
//! EntryHash/
//!   type.hbs
//! String/
//!   TextField/
//!     detail/
//!       render.hbs
//!     edit/
//!       imports.hbs
//!       render.hbs
//!   TextArea/
//!     detail/
//!       render.hbs
//!     edit/
//!       imports.hbs
//!       render.hbs
//!   type.hbs
//! Timestamp/
//!   DateTimePicker/
//!     detail/
//!       render.hbs
//!     edit/
//!       imports.hbs
//!       render.hbs
//!   type.hbs
//! u32/
//!   Slider/
//!     detail/
//!       render.hbs
//!     edit/
//!       imports.hbs
//!       render.hbs
//!   type.hbs
//!
//! As you can see, the top-level folders are the rust types that are possible to use as the field types for an entry. The `type.hbs` file in each of the folders contains the typescript type for that rust type, so that it can be rendered in the frontend.
//!
//! Now, on to the interesting part. Each subfolder in each of the types **corresponds to a frontend widget** that that field can be rendered with. The scaffolding tool will dynamically pick up the folders that exist in that type, and offer the choice to the user to pick a widget from the supported ones in this template. If no widget is found, then the user won't be able to make the field visible in the frontend.
//!
//! After the user has selected a widget, it's up to the templates inside the `entry-type` folder in the root of the template to render the widget, using partials. All the `field-types` directory is registered as partials in the handlebars engine.
//!
//! So for example, if inside the `entry-types` template we have something like this:
//!
//! ```hbs
//! {{> String/TextArea/detail/render }}
//! ```
//!
//! This will get replaced by the contents of the file `field-types/String/TextArea/detail/render.hbs`.
//!
//! ### Instructions
//!
//! Additionally to the folders, you can override the built-in instructions that get shown to the user after each command. The scaffolding tool will look for a file named `<COMMAND>.instructions.hbs` in the folder for the custom template, and if it exists, render its contents and display them to the user. The name of the `COMMAND` for the file matches the names for the folders where the templates for each command exist.
//!
//! This is a great way to guide the users of your template towards next steps that they need to take while using your template.
//!
//! So for example, if there is a `coordinator-zome.instructions.hbs` file in the root folder of your template and the user runs `hc scaffold zome posts --coordinator dnas/forum/zomes/coordinator`, then the scaffolding tool will render its contents and display them to the user when it has finished creating the zome.
//!
//! ### Writing templates
//!
//! The template engine used in the template files is [handlebars](https://handlebarsjs.com/). You can look at its documentation to learn how to write your own templates.
//!
//! Here are the available helpers:
//! - All the built-in helpers described in [the handlebars crate](https://docs.rs/handlebars/latest/handlebars/#built-in-helpers).
//! - Case helpers:
//!   - `pascal_case`: converts the string to pascal case.
//!   - `title_case`: converts the string to title case.
//!   - `lower_case`: converts the string to lower case.
//!   - `snake_case`: converts the string to snake case.
//!   - `camel_case`: converts the string to camel case.
//! - `plural`: converts the given string to its plural.
//! - `concat`: concatenize strings.
//! - `contains`: check whether list contains an element.
//!   - Example usage:
//! ```hbs
//! {{#if (contains entry_type_list "Profile")}}
//! ...
//! {{/if}}
//! ```
//! - `includes`: check whether a string includes a substring.
//!   - Example usage:
//! ```hbs
//! {{#if (includes entry_type.name "Profile")}}
//! ...
//! {{/if}}
//! ```
//! - `merge` and `match_scope`: a pair of helpers useful to add some new code to an already existing code structure, respecting their scope (`{` and `}`) structure.
//!   - `merge`: takes existing code as its only argument.
//!   - `match_scope`: needs to be placed inside a `merge` helper block, and takes the opening of an scope as only argument. It then searches the argument of the `merge` helper for a scope matching that opening of the scope, and replaces its contents with the contents of the `match_scope` block:
//!   - Example usage:
//! ```hbs
//! {{#merge previous_file_content}}
//!   {{#match_scope "export class ExistingClassA {" }}
//!     {{previous_scope_content}} // Variable containing the previous content of the scope
//!
//!     newFunction() {
//!       // This is a new function that will be added at the end of "ExistingClassA"
//!     }
//!   {{/match_scope}}
//!   {{#match_scope "export class ExistingClassB {" }}
//!
//!     {{#merge previous_scope_content}}
//!       {{#match_scope "newFunction() {" }}
//!     {{previous_scope_content}}
//!     // Will add a line at the end of newFunction
//!       {{/match_scope}}
//!     {{/merge}}
//!
//!   {{/match_scope}}
//! {{/merge}}
//! ```

pub mod cli;
pub mod error;
pub mod file_tree;
pub mod reserved_words;
pub mod scaffold;
pub mod templates;
pub mod utils;
pub mod versions;
