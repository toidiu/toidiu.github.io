# AGENTS.md

## Build
- `./scripts/dev_server.sh` — local dev server (script kept up to date)
- `zola build` — production build

## Test
- Run `./scripts/dev_server.sh` and open the URL it prints to verify the site works

## Structure
- Zola static site, TOML frontmatter (`+++` delimiters), Tera templates
- `config.toml` — site config (base_url, taxonomies, menu, sass, search)
- `sass/` — stylesheets
- Content in `content/`

## Content sections
- `blog/` — blog posts, sorted by `date`, tags like `writing`, `thinking`, `crypto`, `rust`, `tls`
- `cucina/cooking/` — recipes, sorted by `date`, tags are cuisine-based (`indian`, `thai`, `french`)
- `projects/` — project pages, sorted by `weight`, use `[extra]` with `image`, `link`, `featured`
- `reads/` — paper summaries, sorted by `date`
- `brag/` — brag doc (single page)
- `fieldbook/` — fieldbook (single page)
- `notes/` — technical notes, sorted by `date`

## Conventions
- All content pages use `[extra]` with `id` matching the template name
- Blog posts and recipes use `[taxonomies].tag` for tags
- Drafts: set `draft = true` in frontmatter
- Summary break: `<!-- more -->` after intro paragraph
- Templates in `templates/`, Sass partials in `sass/`
