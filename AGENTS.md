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
- `personal/recipe/` — recipes, sorted by `date`, tags are cuisine-based (`indian`, `thai`, `french`)
- `personal/career/` — brag/career entries, each file has `[extra]` with `company`, `lp` array
- `personal/career/roles/` — role-based brag entries (uses `blog.html` template)
- `personal/gin/` — gin cocktail recipes
- `projects/` — project pages, sorted by `weight`, use `[extra]` with `image`, `link`, `featured`
- `reads/` — paper summaries, sorted by `date`
- `fieldbook/` — fieldbook (single page)
- `notes/` — technical notes, sorted by `date`

## Conventions
- All content pages use `[extra]` with `id` matching the template name
- Blog posts and recipes use `[taxonomies].tag` for tags
- Drafts: set `draft = true` in frontmatter
- Summary break: `<!-- more -->` after intro paragraph
- Templates in `templates/`, Sass partials in `sass/`

## Making changes
- When renaming/moving a section, update all of:
  - `_index.md` — template, extra.id, extra.name
  - `config.toml` and `dev_config.toml` — menu entries
  - `sass/` — rename file, update import in `main.scss`, update CSS selector
  - `templates/` — template filename must match `template` field in `_index.md`
- Section `_index.md` files use `template`, `page_template`, `sort_by`, `weight` for structure
- Verify with `zola build` or `./scripts/dev_server.sh`
