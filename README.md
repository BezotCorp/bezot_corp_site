# Bezot Corp Site

Official website for Bezot Corp.

## Architecture

The repository separates authored inputs from disposable assembly outputs:

- `site/src/` contains the React application and its stable runtime contracts.
- `site/content/` contains indexed JSON content.
- `bezot_project_assembler/` validates and assembles those inputs.
- `site/prebuild/` is the complete generated TypeScript project.
- `site/prebuild/dist/` is the deployable static website.

Code under `site/src/` must never import `assembled`, `generated`, `prebuild`, or
other build-layer modules. The Rust assembler generates the concrete content,
block registry, and client/server entrypoints under `site/prebuild/assembled/`.

See [docs/site-assembly.md](docs/site-assembly.md) for the ownership boundaries.

## Development

Rust, Node.js, and pnpm are required.

```sh
cargo run --manifest-path bezot_project_assembler/Cargo.toml -- site dev
```

The Rust assembler is the public entrypoint. It creates the initial prebuild,
installs its dependencies, prepares the final site, runs the same checks as
`production`, and starts Vite Preview from the prebuild.

Do not edit `site/prebuild/`; it is disposable assembler output.

## Production

```sh
cargo run --release --manifest-path bezot_project_assembler/Cargo.toml -- site production
```

The assembler creates a self-contained prebuild, prepares the final site,
prerenders every published route, and executes the HTML, SEO, accessibility,
and reference checks.

The deployable output is `site/prebuild/dist/`. In CI, deployment happens only
after this command succeeds.

## Content

Content indexes and documents live under `site/content/`. Block definitions,
field rules, placement rules, runtime ownership, and React component mappings
live in `bezot_project_assembler/datasets/blocks.ron`.

The Rust assembler rejects unknown blocks, unknown properties, invalid property
types, forbidden runtime blocks, invalid placement, and invalid cardinality
before writing the prebuild.
