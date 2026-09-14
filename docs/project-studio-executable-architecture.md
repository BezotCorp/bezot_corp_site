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
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content post get <post-id>
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content post quality <post-id>
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content post save
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content page get <page-id>
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content page save
```

`bezot_project_studio_ui` now edits both content kinds: selecting a page in
the library loads it into a dedicated block editor (add, remove, and reorder
`hero`/`paragraph`/`mail_link`/`card_grid`/`affiliate_callout`/`post_list`
blocks per locale) that saves back through `content page save`.

`content post save` and `content page save` both compare the entry's
previously stored slug (per locale) against the incoming one before writing
anything. If a slug changed, they append a 301 entry from the old path to
the new one to `site/content/redirects.json` (skipped if an entry for that
old path already exists, so a manually curated redirect is never
overwritten). This makes slug changes safe by default: no separate step is
required to avoid turning a published URL into a 404.

### `bezot_project_studio_ui`

`bezot_project_studio_ui` is the Iced studio executable.

It owns the human interface for editing and reviewing site content. It calls
project tools as executables and must not write content JSON directly when a
studio-core command exists for that operation.

The "IA" tab calls `bezot_project_editorial_ai`'s `draft`/`review` commands
the same way — as a subprocess, parsing their `--format json` output — never
by linking editorial logic in-process. Both calls run through `iced::Task`
(the app's first asynchronous work) so a slow local model does not freeze
the UI; `StudioState.ai_task` tracks which one is in flight.

Initial command:

```sh
cargo run --manifest-path bezot_project_studio_ui/Cargo.toml -- site
```

### `bezot_project_editorial_ai`

`bezot_project_editorial_ai` is the editorial automation executable.

It may be called by the studio UI, by cron, or manually. It must use stable
project executable commands for content writes, validation, preview, and final
site preparation.

Initial commands:

```sh
cargo run --manifest-path bezot_project_editorial_ai/Cargo.toml -- site audit
cargo run --manifest-path bezot_project_editorial_ai/Cargo.toml -- site draft --model <name> --topic "<topic>"
cargo run --manifest-path bezot_project_editorial_ai/Cargo.toml -- site review --model <name>
```

`draft` and `review` call a local Ollama server (`http://localhost:11434`),
never a hosted API — `--model` must name a model already pulled locally
(`ollama list`). `draft` asks the model for a bilingual post (title, slug,
SEO description, paragraph per locale) and saves it through
`content post save`, exactly as the studio UI would; the id is prefixed
`ai-editorial-` so `audit` picks it up. `review` fetches every published
post's full content through `content post get` (never reads post JSON
directly) and asks the model for a per-locale SEO score, an update flag, and
concrete suggestions — output only, no writes.

There is no hardcoded default model: pick one that fits your available VRAM
(a 7B–14B quantized model is a reasonable starting point; a 30B+ model may
not leave headroom for anything else running locally).

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

### `common`

`common` is a shared Rust library crate, not a project tool. It is the one
exception to the "no in-process library" rule, because it does not own project
rules or side effects: it only holds data types and small pure utilities that
would otherwise be duplicated across executables.

`common` currently owns:

- `PostEditorState` / `PostLocaleEditor`: the editable post data model, shared
  by `bezot_project_studio_core` (which validates and persists it) and
  `bezot_project_studio_ui` (which edits it in memory before sending it back
  to `bezot_project_studio_core` through the `content post save` command);
- `PostQualityReport`: the editorial scoring logic, so the score shown live in
  the UI and the score returned by `content post quality` are the same
  computation, not two implementations that can drift;
- generic helpers: UUIDs, dates, JSON reading, error constructors, path
  resolution.

Any executable may depend on `common`. No executable may depend on another
executable's crate as a library — that boundary is still process execution
only, per the [Rule](#rule) above.

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

## Known gaps

The following are not implemented yet. They are listed here so new work is not
built on top of an assumed capability that does not exist.

- **No content deletion.** No command or UI action removes a page or post;
  only build output cleanup exists (`bezot_project_assembler/src/prebuild_writer.rs`).
- **No media/image handling in the studio.** Images referenced by content
  (e.g. `ogImage`) must be placed by hand; there is no upload or asset
  command.
- **No real draft preview link.** The "Pilotage" panel in the editor shows an
  editorial quality score, not a URL a reviewer can open to see the rendered
  draft before it is published.
- **No analytics integration** anywhere in the repository.

## Rationale

This architecture keeps callers decoupled from implementation details. The UI,
editorial automation, CI, cron, and humans all use the same executable boundary,
so changing an implementation does not require every caller to be recompiled or
rewired.
