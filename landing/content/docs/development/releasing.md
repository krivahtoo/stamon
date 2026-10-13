+++
title = "Releasing"
description = "How versions, changelogs, images and this site are published."
weight = 5
+++

Releases are automated from [Conventional Commits](https://www.conventionalcommits.org/)
on `main`.

{% <devlab.steps> %}
1. **release-please** keeps a release pull request open. It bumps the version in
   `Cargo.toml` and `Cargo.lock`, and writes the [changelog](@/changelog.md)
   from commit messages.

2. **Merging that pull request** tags the release (`vX.Y.Z`) and creates a
   GitHub release.

3. **The Docker workflow** builds `k4htoo/stamon` for `linux/amd64` and
   `linux/arm64` and pushes the version tags, and `latest`. On a release it
   also publishes `docs/dockerhub.md` as the Docker Hub overview. Every push to
   `main` updates the `edge` tag.

4. **Announce it** if it's worth it: add a blog post and update the site's
   announcement. See [Writing docs](@/docs/development/writing-docs.md#release-announcements).
{% </devlab.steps> %}

## This site

The landing workflow builds `landing/` with Zola on every pull request that
changes it, and deploys it to the `gh-pages` branch on pushes to `main`.
