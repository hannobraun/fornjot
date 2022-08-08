# Release Procedure

This document explains the release procedure. It is only relevant for maintainers.


## 1. Prepare file for release announcement

The release announcement lives on the website, and needs to be created in the [website repository](https://github.com/hannobraun/www.fornjot.app).

We need to make the release announcement available on the website, without publishing it to the blog or RSS feed at first:

1. Create release announcement file: `cargo run -- create-release-announcement`
2. Copy previous release announcement into new file.
3. Update `title` key in the Markdown front matter.
4. Remove the `date` key.
5. Update the `extra.version` key.


## 2. Write release announcement

Use the previously prepared file and create the release announcement:

1. Remove any content left form the previous announcement, leave structure.
2. Go through all pull requests, add them to the draft.
   - Get list of pull requests by executing this command in website repository:
     `cargo run -- print-pull-requests 2022-08-01` (replace date with date of previous release)
   - Thank all contributors
     - new contributors: "special thanks go to first-time contributor `@name`!"
     - other contributors: "thank you, `@name`!"
   - Make notes for introduction
3. Write introduction
   - Ideally, you already have a list of notes from adding the pull requests.
   - Highlight contributions.
4. Update list of sponsors
   - https://github.com/sponsors/hannobraun/dashboard/activity
   - Mention all sponsors at $32 / month and above by name.
5. Write *Issue of the Week*
   See notes below.
6. Write *Outlook*

When done, deploy to the website.

### Issue of the Week

Potential new issues of the week:

- https://github.com/hannobraun/Fornjot/issues/13
- https://github.com/hannobraun/Fornjot/issues/15
- https://github.com/hannobraun/Fornjot/issues/794
- https://github.com/hannobraun/Fornjot/issues/804
- https://github.com/hannobraun/Fornjot/issues/805
- https://github.com/hannobraun/Fornjot/issues/847
- https://github.com/hannobraun/Fornjot/issues/848

Already mentioned issues of the week (remove, once closed):
- 2022-08-08: https://github.com/hannobraun/Fornjot/issues/20
- 2022-08-01: https://github.com/hannobraun/Fornjot/issues/883
- 2022-07-25: https://github.com/hannobraun/Fornjot/issues/856
- 2022-07-18: https://github.com/hannobraun/Fornjot/issues/815
- 2022-07-11: https://github.com/hannobraun/Fornjot/issues/793
- 2022-07-04: https://github.com/hannobraun/Fornjot/issues/479


## 3. Create release branch

In the main Fornjot repository, do this:

```
git switch -c release
```

## 4. Update changelog

Add a changelog entry for the new version. Copy the summary of pull requests from the release announcements and add it to the changelog.

Commit these changes: `git commit -m "Update changelog"`


## 5. Update version

In the release branch, update the version numbers in the `Cargo.toml` files of all crates in the `crates/` directory to the new version. Also update the version numbers of the dependencies between the crates.

Commit these changes: `git commit -m "Update version"`


## 6. Publish the release

Push the release branch, create a pull request, and label it as `release`. Once the CI build completed successfully, merge the pull-request and mention the new version in the commit, e.g. `Release v0.1.2`.

The [release-operator](./tools/release-operator) will run in the scope of GitHub Actions' [CD](./.github/workflows/cd.yml) workflow. It will yield a set of compiled binaries, their checksums and a new GitHub Release with all artifacts attached.

It will also `cargo publish` all crates to [crates.io](https://crates.io/).


## 7. Update release on GitHub

The GitHub Release has been created by automation in the previous step. Copy the Markdown source from the release announcement into the GitHub release. Update as necessary.

Add note up top, linking to the release announcement on the website.


## 8. Finish publishing release announcement

To publish the release on the website properly, add a `date` key to the front matter. Now the announcement should show up on the blog page and the Atom/RSS feed.

Deploy the announcement to the website.


## 9. Promote release announcement

Post the release announcement in the official Fornjot channels:

- Mailing list
- Matrix channel

Post it in the following channels:

- Reddit
  - https://www.reddit.com/r/rust/
    - Use previous post as template
    - Make sure title describes how it relates to Rust
    - Add comment with short explanation, offer to answer questions
- Rust Users
  https://users.rust-lang.org/t/fornjot-code-cad-in-rust-weekly-dev-log/71783
- This Week in Rust
  https://github.com/rust-lang/this-week-in-rust


## 10. Improve release procedure

You are done. Figure out what didn't go optimally, and update this release procedure accordingly.
