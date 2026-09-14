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
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content post delete <post-id>
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content page get <page-id>
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content page save
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content page delete <page-id>
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content media list [--format text|json]
cargo run --manifest-path bezot_project_studio_core/Cargo.toml -- site content media upload <local-path>
```

`content media upload` copies a local image into `site/public/media`, sanitizing
the file name (lowercased, non-alphanumeric characters collapsed to `-`) and
appending a numeric suffix instead of overwriting a colliding name. Only
`png`/`jpg`/`jpeg`/`gif`/`webp`/`svg` extensions are accepted. It prints the
resulting asset as JSON, including the root-relative `publicPath`
(`/media/<name>`) that content fields such as `ogImage` expect — `public/` is
served at the site root by both Vite and the built site, the same way
`favicon.svg` at `site/public/favicon.svg` is served as `/favicon.svg`.

`content post delete` and `content page delete` remove the content file(s),
drop the id from the section's index, and — only if the deleted locale was
`published` — append its route to `site/content/gone-routes.json` (HTTP 410),
so a deleted piece of live content leaves a deliberate signal instead of an
unexplained 404. `content page delete` refuses to delete the page id set as
`homePageId` or the blog's `entryPageId` in `content/index.json`, since
either would break the site build with no obvious pointer back to this rule.
The studio UI's delete button requires a second confirming click before it
sends the command.

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

The "Médias" tab lists uploaded assets (loaded at boot the same way the AI
tab's model catalogue is) and lets the operator pick a local image through a
native file dialog (`rfd`) and upload it through `content media upload`, run
as one `iced::Task` step so the picker and the subprocess call never block
the UI thread. Each asset's public path has a "Copier le chemin" button
(`iced::clipboard::write`) so it can be pasted into a post's or page's
`og_image`/`ogImage` field without retyping it.

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
cargo run --manifest-path bezot_project_editorial_ai/Cargo.toml -- site models
```

`draft`, `review`, and `models` all call a local Ollama server
(`http://localhost:11434`), never a hosted API. `models` lists what is
already pulled (`ollama list`), including each model's on-disk size and
capabilities — the closest available proxy for its VRAM footprint, and the
only reliable way to tell a code-specialized model (fill-in-the-middle
`insert` capability, or a "coder" name) from a general one. `draft` asks the
model for a bilingual post (title, slug, SEO description, paragraph per
locale) and saves it through `content post save`, exactly as the studio UI
would; the id is prefixed `ai-editorial-` so `audit` picks it up. `review`
fetches every published post's full content through `content post get`
(never reads post JSON directly) and asks the model for a per-locale SEO
score, an update flag, and concrete suggestions — output only, no writes.

There is no hardcoded default model, and drafting and reviewing are not
forced to share one: `bezot_project_studio_ui`'s IA tab lets the operator
pick a model per task from the real local catalogue, shows each model's
size against a VRAM budget the operator enters (flagging "too large, will
swap" before they waste a run), and flags code-specialized models as a
weaker fit for prose drafting.

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

## Analytics and consent

`site/index.html` loads Google Tag Manager (container `GTM-WZRQ8MGG`), gated
by the InMobi TCF 2.3 consent banner already present in the same file:

- `gtag('consent', 'default', …)` denies `analytics_storage`/`ad_storage`
  (Google Consent Mode v2) before the GTM snippet loads, so every tag inside
  the container starts in a privacy-safe state.
- A small bridge script listens to InMobi's `__tcfapi` and only calls
  `gtag('consent', 'update', …)` once the visitor has actually granted the
  relevant TCF purposes (purpose 1 plus 7-10 for analytics; 1-4 for ads).

No tag inside the GTM container (GA4 or otherwise) is configured from this
repository — that lives in the Tag Manager web UI, outside this codebase's
reach. This repository is only responsible for loading GTM correctly and
respecting consent before it fires.

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

- **Draft preview is structural, not the site's real rendering.** The
  editors' "Aperçu de lecture" panel composes title/subtitle/paragraphs/
  blocks in order so an author can proofread flow before publishing, but it
  is not the site's actual CSS-styled output. Prerendering excludes
  unpublished content entirely (see `project-checks.md`), so there is no
  live URL for a draft even for a human author today — building one would
  mean assembling an isolated copy of the site with the draft temporarily
  marked published, which is a larger, separate piece of work.

## Rationale

This architecture keeps callers decoupled from implementation details. The UI,
editorial automation, CI, cron, and humans all use the same executable boundary,
so changing an implementation does not require every caller to be recompiled or
rewired.
