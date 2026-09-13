# Project studio executable architecture

## Rule

Every callable project tool is an executable.

Callers must cross tool boundaries through process execution and stable command
interfaces. They must not import another project tool as an in-process Rust
library.

```text
caller
  -> executable command
  -> structured stdout/stderr/status/files
```

This rule applies to human terminal usage, the studio UI, cron jobs, editorial
automation, and CI.

## Executables

### `bezot_project_studio_core`

`bezot_project_studio_core` is the central project operations executable.

It owns stable commands for:

- content creation and updates;
- content validation;
- final site preparation;
- preview startup;
- production preparation;
- coordination with lower-level project tools.

It is not a Rust library for the UI. The UI and other callers execute it as a
process.

Initial commands:

```sh
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site dev
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site production
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content list --format text
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content list --format json
```

### `bezot_project_studio_ui`

`bezot_project_studio_ui` is the Iced studio executable.

It owns the human interface for editing and reviewing site content. It calls
project tools as executables and must not write content JSON directly when a
studio-core command exists for that operation.

Initial command:

```sh
cargo run --manifest-path bezot_project_studio_ui/Cargo.toml -- site
```

### `bezot_project_editorial_ai`

`bezot_project_editorial_ai` is the editorial automation executable.

It may be called by the studio UI, by cron, or manually. It must use stable
project executable commands for content writes, validation, preview, and final
site preparation.

Initial command:

```sh
cargo run --manifest-path bezot_project_editorial_ai/Cargo.toml -- site audit
```

### `bezot_project_assembler`

`bezot_project_assembler` remains an executable project tool.

It owns the technical assembly boundary:

- source and content scanning;
- block/content validation;
- prebuild materialization;
- final site preparation support;
- preview support when called directly.

Other tools may call it as a process. It must not be treated as an in-process
library unless a future architecture document explicitly changes this rule.

## Command ownership

Commands should be named after the operation they guarantee, not after where
they run.

Required concepts:

- `dev`: prepare the final site, run checks, then start Vite Preview.
- `production`: prepare the final site and run checks; CI may publish only after
  this command succeeds.
- `prepare final site`: common operation that assembles, builds, prerenders, and
  checks the final output.

Do not describe the common operation as "local" or as "production-only". It is
the same final-site preparation used before preview and before publication.

## Publication

Preparing the final site does not publish by itself.

CI publication is a separate step:

```text
production command succeeds
  -> CI deploys site/prebuild/dist
```

## Studio UI boundary

The UI is an operator, not the owner of project rules.

It may:

- list content and project state by calling executable commands;
- request content writes through executable commands;
- start preview through executable commands;
- display validation/check results.

It must not:

- duplicate block validation rules;
- write JSON content directly when a command exists;
- import executable crates as libraries;
- bypass final-site checks before preview or production.

## Editorial automation boundary

Editorial automation follows the same rules as the UI.

It may create or update drafts only through stable executable commands. Cron and
UI-triggered editorial actions must therefore share the same validation and
write path.

## Rationale

This architecture keeps callers decoupled from implementation details. The UI,
editorial automation, CI, cron, and humans all use the same executable boundary,
so changing an implementation does not require every caller to be recompiled or
rewired.
