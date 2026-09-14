# Site assembly boundaries

## Pipeline

```text
site/src + site/content + site configuration
                    |
                    v
       bezot_project_assembler
                    |
                    v
          site/prebuild
                    |
                    v
        TypeScript + Vite + SSR
                    |
                    v
       site/prebuild/dist
```

The assembler is the operational entrypoint. Its public commands intentionally
match the historical site interface: `production` prepares and checks the final
site output, and `dev` runs that same final-site preparation before starting
Vite Preview from the assembled prebuild.

## Authored source

`site/src/` owns the application behavior and presentation:

- React components and layouts;
- routing and SEO behavior;
- stable content and runtime contracts;
- the runtime factory that operates on injected content.

It does not own concrete content, a generated singleton, or build entrypoints.
Imports from assembly or build layers are forbidden and checked whenever the
final site is prepared.

## Rust assembler

`bezot_project_assembler/` owns the boundary between authored inputs and an
executable project:

- indexed JSON loading and validation;
- block vocabulary, field types, placement, cardinality, and source ownership;
- mapping block names to authored React components;
- deterministic, split TypeScript content modules;
- client and SSR composition entrypoints;
- source, scripts, public assets, content, and configuration materialization;
- incremental output tracking and deletion;
- preparing the final site and launching Vite Preview from the prebuild.

The assembler does not own React rendering behavior or final bundling. It
connects validated content to authored production code, then delegates
transpilation and bundling to TypeScript and Vite.

## Prebuild

`site/prebuild/` is fully reproducible and must not be edited. It contains:

- copied authored application code;
- copied build support, public assets, and validated source content;
- generated content modules under `assembled/content/`;
- a generated block registry derived from `blocks.ron`;
- generated client and server entrypoints;
- all configuration required to produce `dist/`.

Neither the build nor prerendering reads files outside this directory after
assembly completes.
