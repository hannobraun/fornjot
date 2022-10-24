# Release Procedure

This document explains the release procedure. It is only relevant for maintainers.


## 1. Install/update automation tool

We have a CLI tool to automate parts of the release process. Make sure you have the latest version of that installed.

In the Fornjot repository, run:
``` sh
cargo install --path tools/automator
```


## 2. Write release announcement

The release announcement lives on the website, and needs to be created in the [website repository](https://github.com/hannobraun/www.fornjot.app).

First, create the initial draft of the release announcement:

```
automator announcement
```

This will create an announcement that will initially not be published to the blog or RSS feed, but it will be available at its final URL.

Now, edit this file to finish the release announcement:

1. Add pull requests
   A list of pull requests has been added to the draft automatically. Go through them, update and merge entries as appropriate, sort them into the correct categories.
2. Write introduction
   - Summarize the week's development.
   - Highlight contributions.
3. Choose title and subtitle.

When done, deploy to the website.


## 3. Create release branch

In the main Fornjot repository, do this:

```
git switch -c release
```

## 4. Update changelog

Add a changelog entry for the new version. Copy the summary of pull requests from the release announcements and add it to the changelog.

Commit these changes: `git commit -m "Update changelog"`


## 5. Update version

Update all `version` keys in the top-level `Cargo.toml`.

Commit these changes: `git commit -m "Update version"`


## 6. Publish the release

Push the release branch:
``` sh
git push -u origin release
```

Create a pull request:

- Mention the version in the title (e.g. `Release v0.1.2`).
- Label it as `release`.
- Lock the pull request. Anything that updates this pull request (like comments), could confuse next week's release automation (because GitHub doesn't allow sorting pull requests by merge date, unfortunately).

Once the CI build completed successfully, merge the pull request. The release automation will now compile binaries, create a release on GitHub, and publish to [crates.io](https://crates.io/).


## 7. Update release on GitHub

The GitHub Release has been created by automation in the previous step. Copy the Markdown source from the release announcement into the GitHub release. Update as necessary.

Add note up top, linking to the release announcement on the website.


## 8. Finish publishing release announcement

To publish the release on the website properly, add a `date` key to the front matter. Now the announcement should show up on the blog page and the Atom/RSS feed.

Deploy the announcement to the website.


## 9. Promote release announcement

Post the release announcement in the official Fornjot community channels:

- Mailing list
- Matrix channel

Additionally, post it in the following places:

- Twitter
  https://twitter.com/hannobraun
- Mastodon
  https://fosstodon.org/@hannobraun
- Rust Users
  https://users.rust-lang.org/t/fornjot-code-cad-in-rust-weekly-dev-log/71783


## 10. Improve release procedure

You are done. Figure out what didn't go optimally, and update this release procedure accordingly.
