//! # Using the scaffolding CLI tool:
//!
//! After having installed it, these are the commands that are available:
//!
//! ```bash
//! // Scaffold an empty web-app, using a built-in template
//! hc-scaffold web-app forum
//!
//! cd forum
//!
//! // Scaffold a dna inside the newly scaffolded app
//! hc-scaffold dna forum
//!
//! // Scaffold a zome inside the newly scaffolded dna
//! hc-scaffold zome posts
//!
//! // Scaffold an entry-type inside the newly scaffolded zome
//! hc-scaffold entry-type post
//!
//! // Scaffold an collection for the newly scaffolded entry-type
//! hc-scaffold collection global all_posts
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
//! - Vanilla
//! - Lit
//! - Vue
//! - Svelte
//!
//! These templates provide most of the skeleton you need to start your own holochain app.
//!
//! But! They are not complete, nor provide good design from the UI/UX perspective. They are trying to be unopinionated in that regard, so that you as the developer can apply your own style of building frontend apps.
//!
//! To allow for more flexibility, the scaffolding tool can be extended and customized using custom templates. This would allow you to create a "React" template, or a "Vue + tailwind" template, or whatever style of frontend code and packaging you want for your app.
//!
//! ## Using custom templates
//!
//! If you know of some already existing template that you want to use, simply scaffold a new web-app pointing to its git repository with:
//!
//! `hc-scaffold web-app forum --templates-url https://github.com/holochain-open-dev/templates`
//!
//! If instead, you want to use that template within an already existing repository, you can use this command instead:
//!
//! `hc-scaffold template get https://github.com/holochain-open-dev/templates`
//!
//! Both of the previous commands will create a `.templates` folder in the root folder of your repository, with a copy of the template.
//!
//! From this point on, any command that you execute with the scaffolding tool is going to use that custom template instead of the built-in ones.
//!
//! If later on the template adds some new features and you want to include them in your repository, you can just run this command again:
//!
//! `hc-scaffold template get https://github.com/holochain-open-dev/templates`
//!
//! And select "Merge with existing template", to overwrite the old one.
//!
//! ## How to create a custom template
//!
//! Creating and maintaining your own template can be challenging at first, so look for existing templates that you can reuse before diving in to create your own.
//!
//! The best way to start creating a custom template is to go from one of the built-in ones, and modify it.
//!
//! To create a custom template, run this command and select the built-in template that you want to go from, and also give a name to your template.
//!
//! `hc-scaffold template init`
//!
//! At this point, you'll have a `.templates/<TEMPLATE NAME>` folder. That's where your custom template lives.
//!
//! Templates have this directory structure:
//!
//! ```
//! coordinator-zome/
//! dna/
//! entry-type/
//! field-types/
//! collection/
//! integrity-zome/
//! link-type/
//! web-app/
//! ```
//!
//! Each folder corresponds to the templates that are going to be created when running a specific command. This is the steps that are executed:
//!
//! 1. The user executes a scaffolding command, like `hc-scaffold web-app`.
//! 2. The scaffolding tool asks the user to input all the necessary information.
//! 3. The apropriate **backend** and **testing** code is created automatically, the custom template can't influence it.
//! 4. The scaffolding tool looks for a custom template in the `.templates` folder.
//! 5. If there is one, it will look for a folder inside that custom template that corresponds to the command being run.
//!   - Eg. `hc-scaffold web-app` will look for a folder named `web-app`.
//! 6. If there is one, it will copy the directory structure inside that folder and select the files with the `.hbs` extension.
//! 7. It will render the contents of each of the files using as context the appropriate data from the command.
//!   - Eg. in `hc-scaffold web-app`, one of the fields of the context is `app_name`, which is the name of the app that the user input.
//! 8. Lastly, it will merge the resulting directory structure with the existing repository structure. If a file already existed, it will overwrite its contents.
//!
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
//!
//! ### Field types
//!
//! The `field-types` folder is special. It has the following directory structure:
//!
//! ```
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
//! ```
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
//! So for example, if there is a `coordinator-zome.instructions.hbs` file in the root folder of your template and the user runs `hc-scaffold zome posts --coordinator dnas/forum/zomes/coordinator`, then the scaffolding tool will render its contents and display them to the user when it has finished creating the zome.
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
//! - `merge_scope`: takes existing code as its first argument, and the opening of an scope as its second. It then replaces the contents of that scope with the contents of the block:
//!   - Example usage:
//! ```hbs
//! {{#merge_scope previous_file_content "export class ExistingClass {" }}
//!   {{previous_scope_content}} // This will be replaced with the existing content of the scope
//!
//!   newFunction() {
//!     // This is a new function that will be added at the end of "ExistingClass"
//!   }
//! {{/merge_scope}}
//! ```

pub mod cli;
pub mod error;
pub mod file_tree;
pub mod scaffold;
pub mod templates;
pub mod utils;
pub mod versions;
