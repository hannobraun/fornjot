# Release Procedure

This document explains the release procedure. It is only relevant for maintainers.


## 1. Create release branch: `git switch -c v0.1.2`

Replace `0.1.2` in that command with the actual version.


## 2. Update changelog

Add a changelog entry for the new version. Use the entry for the previous version as a template. Remember to thank first-time contributors.

The list of pull request since the last release is available here:
https://github.com/hannobraun/Fornjot/pulls?q=is%3Apr+is%3Aclosed+merged%3A%3E2022-01-27

Replace `2022-01-27` at the end of that URL with the day before the previous release.

For the version after `0.5.0`, add the following to the changelog:
- Replace "Host Application" title with crate name: `fj-app`
- Convert title of each crate section into link to crate on crates.io.
- Add one-sentence description of the respective crate to each section.
- Link to all Weekly Dev Logs that cover the release.

Update these instructions, once this has been done.

Commit these changes: `git commit -m "Update changelog"`


## 3. Write release announcement

Use the previous release announcement as a template. Once finished, publish it on the website, such that it is reachable under the correct URL, but not listed on the Blog page.

- Figure out how to do that, document it here.
- In addition to what's in the last release announcement, add a call to report bugs near the top, right below the summary.

Remember to thank everyone who contributed to the release.


## 4. Update versions

In the release branch, update the version numbers in the `Cargo.toml` files of all crates to the new version. Also update the version numbers of the dependencies between the crates.


## 5. Publish the release

Push the release branch and create a pull request and label it as `release`. Once the CI build completed successfully, merge the pull-request and mention the new version in the commit, e.g. `Release v0.1.2`.

The [release-operator](./release-operator) will run in the scope of GitHub Actions' [CD](./.github/workflows/cd.yml) workflow. It will yield a set of compiled binaries, their checksums and a new GitHub Release with all artifacts attached.

Next, publish the release by running `cargo publish` for each crate.


## 6. Update release on GitHub

The GitHub Release has been created by automation in the previous step. Use the previous release as a template to populate its body.

Make the following changes, compared to the release for version `0.5.0`:
- Link to release announcement on the website at the top. Mention that people can subscribe there.
- Copy the full content of the release announcement.

Once this has been done, update these instructions.


## 7. Finish publishing release announcement

Make it so that is appears on the Blog page.

Figure out how to do that, then publish the instructions here.


## 8. Promote release announcement

Post the release announcement on the following sites:

- /r/rust: https://www.reddit.com/r/rust/
- Rust Users: https://users.rust-lang.org/
- This Week in Rust: https://github.com/rust-lang/this-week-in-rust

Use the previous release announcement posted there as a template.


## 9. Improve release procedure

You are done. Figure out what didn't go optimally, and update this release procedure accordingly.
