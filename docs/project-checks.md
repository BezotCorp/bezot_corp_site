# Project checks

This document lists the checks currently enforced by the project.

## Source and content

The project checks:

- source code linting
- content index consistency
- page declarations
- locale declarations
- blog post declarations
- publication status consistency
- generated site data creation

## Build and prerendering

The project checks:

- TypeScript compilation
- client production build
- SSR build for prerendering
- static HTML prerendering
- cleanup of the temporary SSR output

## Generated output

The project checks that the generated dist output:

- exists
- contains the required public files
- does not contain forbidden output paths
- does not contain forbidden public text
- contains generated HTML files for published pages
- excludes generated HTML files for non-published pages
- contains generated HTML files for published blog posts
- excludes generated HTML files for non-published blog posts

## Sitemap

The project checks that the sitemap:

- uses the configured canonical host
- points only to existing generated HTML files
- lists public generated HTML files

## SEO metadata

The project checks that generated HTML pages have:

- a non-empty title
- a non-empty meta description
- a canonical URL
- a canonical URL matching the generated route

## HTML structure

The project checks that generated public HTML has:

- an html lang attribute
- exactly one h1 per page
- image alt attributes
- non-empty aria-label values
- no focusable element hidden with aria-hidden="true"
- links with href attributes
- links with accessible text
- buttons with accessible text
- internal links pointing to existing generated targets
