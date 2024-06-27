# Holochain Scaffolding CLI Documentation

## Overview

A command-line interface for creating and modifying a Holochain application (hApp).

## General Usage

```bash
hc-scaffold [OPTIONS] <SUBCOMMAND>
```

### Flags

- `-h`, `--help`  
  Prints help information.

- `-V`, `--version`  
  Prints version information.

### Options

- `-t`, `--template <template>`  
  The template to use for the `hc-scaffold` commands. Can either be an option from the built-in templates: "vanilla", "vue", "lit", "svelte", "react", "headless" or a path to a custom template.

### Subcommands

- `collection`  
  Scaffold a collection of entries in an existing zome.
  
- `dna`  
  Scaffold a DNA into an existing app.
  
- `entry-type`  
  Scaffold an entry type and CRUD functions into an existing zome.
  
- `example`  
  Scaffold an example hApp.
  
- `link-type`  
  Scaffold a link type and its appropriate zome functions into an existing zome.
  
- `template`  
  Manage custom templates.
  
- `web-app`  
  Scaffold a new, empty web app.
  
- `zome`  
  Scaffold one or multiple zomes into an existing DNA.

- `help`  
  Prints this message or the help of the given subcommand(s).

## Subcommand Details

### `hc-scaffold collection`

Scaffold a collection of entries in an existing zome.

**Usage:**

```bash
hc-scaffold collection [FLAGS] [OPTIONS] [ARGS]
```

#### Flags

- `-h`, `--help`  
  Prints help information.

- `--no-ui`  
  Skips UI generation for this collection.

- `-V`, `--version`  
  Prints version information.

#### Options

- `--dna <dna>`  
  Name of the DNA in which you want to scaffold the zome.

- `--zome <zome>`  
  Name of the integrity zome in which you want to scaffold the link type.

#### Arguments

- `<collection-type>`  
  Collection type: "global" or "by-author".

- `<collection-name>`  
  Collection name, just to differentiate it from other collections.

- `<entry-type>`  
  Entry type that is going to be added to the collection.

### `hc-scaffold dna`

Scaffold a DNA into an existing app.

**Usage:**

```bash
hc-scaffold dna [OPTIONS] [name]
```

#### Flags

- `-h`, `--help`  
  Prints help information.

- `-V`, `--version`  
  Prints version information.

#### Options

- `--app <app>`  
  Name of the app in which you want to scaffold the DNA.

#### Arguments

- `<name>`  
  Name of the DNA being scaffolded.

### `hc-scaffold entry-type`

Scaffold an entry type and CRUD functions into an existing zome.

**Usage:**

```bash
hc-scaffold entry-type [FLAGS] [OPTIONS] [--] [name]
```

#### Flags

- `-h`, `--help`  
  Prints help information.

- `--no-ui`  
  Skips UI generation for this entry-type, overriding any specified widgets in the `--fields` option.
  
  **WARNING:** Opting out of UI generation for an entry type but not for other entry types, link types, or collections associated with it may result in potential UI inconsistencies. Specifically, UI elements intended for associated entry types, link types, or collections could inadvertently reference or expect elements from the skipped entry type.
  
  If you choose to use this flag, consider applying it consistently across all entry-type, link-type, and collection scaffolds within your project to ensure UI consistency and avoid the outlined integration complications.

- `-V`, `--version`  
  Prints version information.

#### Options

- `--crud <crud>`  
  The Create, "Read", "Update", and "Delete" zome call functions that should be scaffolded for this entry type. If `--reference-entry-hash` is `true`, only "Create" and "Read" will be scaffolded.

- `--dna <dna>`  
  Name of the DNA in which you want to scaffold the zome.

- `--fields <fields>...`  
  The fields that the entry type struct should contain.  
  **Grammar:** `<FIELD_NAME>:<FIELD_TYPE>:<WIDGET>:<LINKED_FROM>`, (widget and linked_from are optional)  
  **Example:** `"title:String:TextField"`, `"posts_hashes:Vec\<ActionHash\>::Post"`

- `--link-from-original-to-each-update <link-from-original-to-each-update>`  
  Whether to create a link from the original entry to each update action. Only applies if update is selected in the `crud` argument.

- `--reference-entry-hash <reference-entry-hash>`  
  Whether this entry type should be referenced with its "EntryHash" or its "ActionHash". If referred to by "EntryHash", the entries can't be updated or deleted.

- `--zome <zome>`  
  Name of the integrity zome in which you want to scaffold the entry definition.

#### Arguments

- `<name>`  
  Name of the entry type being scaffolded.

### `hc-scaffold link-type`

Scaffold a link type and its appropriate zome functions into an existing zome.

**Usage:**

```bash
hc-scaffold link-type [FLAGS] [OPTIONS] [ARGS]
```

#### Flags

- `-h`, `--help`  
  Prints help information.

- `--no-ui`  
  Skips UI generation for this link type.

- `-V`, `--version`  
  Prints version information.

#### Options

- `--bidirectional <bidirectional>`  
  Whether to create the inverse link, from the `--to-referenceable` entry type to the `--from-referenceable` entry type.

- `--delete <delete>`  
  Whether this link type can be deleted.

- `--dna <dna>`  
  Name of the DNA in which you want to scaffold the zome.

- `--zome <zome>`  
  Name of the integrity zome in which you want to scaffold the link type.

#### Arguments

- `<from-referenceable>`  
  Entry type (or agent role) used as the base for the links.

- `<to-referenceable>`  
  Entry type (or agent role) used as the target for the links.

### `hc-scaffold template`

Manage custom templates.

**Usage:**

```bash
hc-scaffold template <SUBCOMMAND>
```

#### Flags

- `-h`, `--help`  
  Prints help information.

- `-V`, `--version`  
  Prints version information.

#### Subcommands

- `clone`  
  Clone the template in use into a new custom template.

- `help`  
  Prints this message or the help of the given subcommand(s).

### `hc-scaffold template clone`

Clone the template in use into a new custom template.

**Usage:**

```bash
hc-scaffold template clone [FLAGS] [OPTIONS]
```

#### Flags

- `-h`, `--help`  
  Prints help information.

- `-V`, `--version`  
  Prints version information.

#### Options

- `--to-template <to-template>`  
  The folder to initialize the template into, will end up at `<TO TEMPLATE>`.

### `hc-scaffold web-app`

Scaffold a new, empty web app.

**Usage:**

```bash
hc-scaffold web-app [FLAGS] [OPTIONS] [ARGS]
```

#### Flags

- `-F`, `--disable-fast-track`  
  Whether to skip setting up an initial DNA and its zome(s) after the web app is scaffolded.

- `-h`, `--help`  
  Prints help information.

#### Options

- `--setup-nix`  
  Whether to setup the holonix development environment for this web app.

- `-p`, `--package-manager <package-manager>`  
  The package manager to use for scaffolding the web app. Can be one of the following: "bun", "npm", "pnpm", or "yarn". When a lockfile is detected, the respective package manager will be used as the default value; otherwise, npm will be set as the default.  
  **Default:** `npm`

#### Arguments

- `<name>`  
  Name of the app to scaffold.

- `<description>`  
  Description of the app to scaffold.

### hc-scaffold zome

Scaffold one or multiple zomes into an existing DNA.

**Usage:**

```bash
hc-scaffold zome [FLAGS] [OPTIONS] [ARGS]
```

#### Flags

- `-h`, `--help`  
  Prints help information.

- `-V`, `--version`  
  Prints version information.

#### Options

- `--coordinator <coordinator>`  
  Scaffold a coordinator zome at the given path.

- `--dna <dna>`  
  Name of the DNA in which you want to scaffold the zome.

- `--integrity <integrity>`  
  Scaffold an integrity zome at the given path.

#### Arguments

- `<name>`  
  Name of the zome being scaffolded.

### `hc-scaffold example`

Scaffolds an example Holochain application to help you get started quickly

**Usage**

```bash
hc-scaffold example [FLAGS] [ARGS]
```

#### Flags

- `-h`, `--help`  
  Prints help information.

- `-V`, `--version`  
  Prints version information.

- `-p`, `--package-manager <package-manager>`  
  The package manager to use for scaffolding the example. Can be one of the following: "bun", "npm", "pnpm", or "yarn". When a lockfile is detected, the respective package manager will be used as the default value; otherwise, npm will be set as the default.  
  **Default:** `npm`

#### Arguments

- `<example>`
  The name of the example to scaffold. Available options are:
  - `hello-world`
  - `forum`