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

First, create the initial draft of the release announcement (replace date with date of previous release; replace version with version of the new release):

```
automator create-release-announcement 2022-08-22 0.13.0
```

This will create an announcement that will initially not be published to the blog or RSS feed, but it will be available at its final URL.

Now, edit this file to finish the release announcement:

1. Add pull requests
   - A list of pull requests has been added to the draft automatically.
     Go through them, update and merge entries as appropriate, sort them into the correct categories.
   - Thank all contributors
     - new contributors: "special thanks go to first-time contributor `@name`!"
     - other contributors: "thank you, `@name`!"
   - Make notes for introduction
2. Write introduction
   - Ideally, you already have a list of notes from adding the pull requests.
   - Highlight contributions.
3. Update list of sponsors
   - https://github.com/sponsors/hannobraun/dashboard/activity
   - Mention all sponsors at $32 / month and above by name.
4. Write *Issue of the Week*
   See notes below.
5. Write *Outlook*

When done, deploy to the website.

### Issue of the Week

Potential new issues of the week:

- https://github.com/hannobraun/Fornjot/issues/13
- https://github.com/hannobraun/Fornjot/issues/15
- https://github.com/hannobraun/Fornjot/issues/794
- https://github.com/hannobraun/Fornjot/issues/805
- https://github.com/hannobraun/Fornjot/issues/821
- https://github.com/hannobraun/Fornjot/issues/848
- https://github.com/hannobraun/Fornjot/issues/851
- https://github.com/hannobraun/Fornjot/issues/920
- https://github.com/hannobraun/Fornjot/issues/937
- https://github.com/hannobraun/Fornjot/issues/971
- https://github.com/hannobraun/Fornjot/issues/986
- https://github.com/hannobraun/Fornjot/issues/987
- https://github.com/hannobraun/Fornjot/issues/996
- https://github.com/hannobraun/Fornjot/issues/997

Already mentioned issues of the week (remove, once closed):
- 2022-08-29: https://github.com/hannobraun/Fornjot/issues/980
- 2022-08-22: https://github.com/hannobraun/Fornjot/issues/938
- 2022-08-08: https://github.com/hannobraun/Fornjot/issues/20
- 2022-08-01: https://github.com/hannobraun/Fornjot/issues/883
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

Push the release branch:
``` sh
git push -u origin release
```

Create a pull request, mention the release in the title (e.g. `Release v0.1.2`), and label it as `release`.

- Once the CI build completed successfully, merge the pull request.
- After merging, lock the pull requests. Anything that updates this pull request (like comments), could confuse next week's release automation (because GitHub doesn't allow fetching pull requests by merged date, unfortunately).

The release automation will now compile binaries, create a release on GitHub, and publish to [crates.io](https://crates.io/).


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

Additionally, post it in the following communities:

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
