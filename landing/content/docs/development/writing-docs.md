+++
title = "Writing docs"
description = "Where documentation goes and how to write it."
weight = 7
+++

This site lives in `landing/` and is built with [Zola](https://www.getzola.org/)
0.23.6 and the vendored [DevLab theme](https://codeberg.org/RiPetitor/devlab-theme)
in `landing/themes/devlab-theme/`. Don't edit the theme; override it from the
site instead.

## Preview

```sh
cd landing
zola serve
```

Before opening a pull request, run:

```sh
zola check --skip-external-links
```

The landing workflow runs the same check on every pull request that touches
`landing/`.

## Where pages go

| Section | Put here |
|---|---|
| `docs/getting-started/` | Installing Stamon and the first steps after |
| `docs/monitors/` | One page per check type |
| `docs/features/` | One page per feature: alerts, maintenance, status pages, roles |
| `docs/self-hosting/` | Running a server: configuration, security, upgrades, data |
| `docs/reference/` | Exact formats: the HTTP API and alert payloads |
| `docs/development/` | Building, architecture and contributing |
| `blog/` | Release announcements and other dated posts |

When you add a check type or channel, add or update its page in the same pull
request. A new top-level section also needs an entry in the `navigation` list
under `[extra.devlab.collections.docs]` in `zola.toml`.

The changelog (`content/changelog.md`) is written by release-please; don't edit
it by hand.

## Page conventions

- Every page has `title`, `description` and `weight` in its front matter. The
  weight sets its order in the sidebar.
- Every section has an `_index.md` with `sort_by = "weight"`,
  `template = "docs.html"` and `page_template = "doc-page.html"`.
- Start headings at `##`; the page title is the `h1`. `##` and `###` headings
  appear in the "On this page" panel.
- Give a heading an explicit id, like {% raw %}`## Channels {#channels}`{% endraw %}, when other
  pages link to it.
- Link to other pages with `@/` paths, like `[Alerts](@/docs/features/alerts.md)`.
  `zola check` then catches broken links.
- When you move or rename a page, add its old URL to `aliases` so links keep
  working.
- Keep pages to one topic. Split a page once it covers several things readers
  look for separately.

## Components

DevLab's components work in Markdown:

{% raw %}
```jinja
{% <devlab.callout type="warning" title="Back up first"> %}
Warnings, tips and notes.
{% </devlab.callout> %}

{% <devlab.steps> %}
1. A numbered walkthrough.
2. One list item per step.
{% </devlab.steps> %}
```
{% endraw %}

Use tabs for the same task done different ways, such as Docker, Compose and
from source. See the
[theme's component reference](https://ripetitor.codeberg.page/devlab-theme/docs/reference/components/)
for all of them.

## Release announcements

For a release worth announcing:

1. Add a post in `content/blog/` with `[extra] release = "vX.Y.Z"`.
2. Point `[extra.devlab.announcement]` in `zola.toml` at it, and change `key`
   so people who dismissed the last banner see the new one.
