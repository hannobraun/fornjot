# Release Procedure

This document explains the release procedure. It is only relevant for maintainers.


## 1. Create release branch

```
git switch -c release
```

## 2. Update changelog

Add a changelog entry for the new version. Use the entry for the previous version as a template. Remember to thank the contributors.

The list of pull request since the last release is available here:
https://github.com/hannobraun/Fornjot/pulls?q=is%3Apr+merged%3A%3E2022-01-25+sort%3Acreated-asc

Replace the date near the end of that URL with the day before the previous release.

Commit these changes: `git commit -m "Update changelog"`


## 3. Write release announcement

Use the previous release announcement as a template, but make the following changes:

1. Place the directory in the `static/blog/` instead of `content/blog/`.
2. Move the `index.md` file to `content/release.md`.
2. Remove the `date` key from the Markdown front matter.
3. Add a `path` key to explicitly specify the path.

The path should be identical to where the page would be, if it were added to `content/blog/`. This will result in the page being and all static assets being available at their final URLs, without the page being listed on the blog page, or being picked up by the Atom/RSS feed.

Deploy the release announcement in this form to the website.


## 4. Update versions

In the release branch, update the version numbers in the `Cargo.toml` files of all crates to the new version. Also update the version numbers of the dependencies between the crates.


## 5. Publish the release

Push the release branch, create a pull request, and label it as `release`. Once the CI build completed successfully, merge the pull-request and mention the new version in the commit, e.g. `Release v0.1.2`.

The [release-operator](./tools/release-operator) will run in the scope of GitHub Actions' [CD](./.github/workflows/cd.yml) workflow. It will yield a set of compiled binaries, their checksums and a new GitHub Release with all artifacts attached.

It will also `cargo publish` all crates to [crates.io](https://crates.io/).


## 6. Update release on GitHub

The GitHub Release has been created by automation in the previous step. Copy the release announcement from the website (in HTML form), using the following procedure:

1. Take contents of `<main>`
2. Replace `href="/` with `href="https://www.fornjot.app/`.
3. Replace `src="/` with `src="https://www.fornjot.app/`.
4. Added note up top, linking to the release announcement on the website.


## 7. Finish publishing release announcement

To publish the release on the website properly, do the following:

1. Move the directory with the static assets to `content/blog/`.
2. Move `content/release.md` to the directory with the other assets, as `index.md`.
3. Add the `date` key to the front matter.
4. Remove the `path` key.

Now the announcement should show up on the blog page and the Atom/RSS feed.

Deploy the announcement to the website.


## 8. Promote release announcement

Post the release announcement in the official Fornjot channels:

- Matrix channel
- Mailing list

Post it on the following sites:

- /r/rust: https://www.reddit.com/r/rust/
- Rust Users: https://users.rust-lang.org/
- This Week in Rust: https://github.com/rust-lang/this-week-in-rust

Use the previous release announcement posted there as a template.


## 9. Improve release procedure

You are done. Figure out what didn't go optimally, and update this release procedure accordingly.
