# Contribution Guide

## Introduction

Thank you for your interest in contributing to Fornjot. I appreciate the help!

This document teaches you how to...

- ...use Fornjot as a developer.
- ...report a bug.
- ...suggest an improvement
- ...make an improvement.
- ...find work to do.

Each of these topics is addressed in one of the following sections. After that, there's also a collection of optional guidelines you can follow when making your contribution.


## Using Fornjot

### Displaying models

The README explains [how to to use Fornjot to run the example models](README.md#usage).

### Running tests

You can run the full suite of unit/integration tests like this:

``` sh
cargo test
```

During development, it can often be convenient to just run the tests for the crate you're currently working on, as that can be much quicker. You can run only the tests for `fj-core`, for example, like this:

``` sh
cargo test -p fj-core
```

### Running a full build

You can run the full suite of checks and tests like this:

``` sh
just ci
```

You can also run a lighter version that just checks for code correctness:
``` sh
just test
```

This requires [`just`](https://crates.io/crates/just), which you can install like this:

``` sh
cargo install just
```

There might be additional setup required, depending on the platform you develop on. For example, the following [was reported](https://github.com/hannobraun/Fornjot/pull/1342#issue-1447029944):

- > To make `just build` work on Windows I had to install [`Visual Studio` with `clang`](https://www.wikihow.com/Install-Clang-on-Windows) and then [`MinGW` with `ar`](https://winlibs.com/) and add both tools to `PATH` env var to make `cross-compiler` work.
- > On Linux, I had to install `libfontconfig-dev` and `clang`.

The purpose of `just ci` is to run the same suite of checks and tests that the CI build runs, so you can figure out any issues in advance, without having to submit a pull request, and having to wait for a CI run to finish.

This is defined in [`justfile`](justfile). Please note that `justfile` is maintained in parallel to the CI configuration. Most deviations should be considered bugs in `justfile`.


## Reporting Bugs

To report a bug, please [open an issue](https://github.com/hannobraun/Fornjot/issues/new) in Fornjot's GitHub repository.

Feel free to first check the [list of open issues][issues], and if you find an existing one for your bug, add your voice there. If you're not sure or don't have the time, **don't worry, just open an issue**. I'd rather deal with duplicate issues than not hear about a bug at all.


## Suggesting Improvements

There are two ways to suggest an improvement, like a new feature or changes to an existing one:

- [Open an issue][issues]
- Start a discussion on [Matrix] or [Discussions]

We use issues to track work that is mostly actionable, so stuff that someone can work on right now, or at least soon. Having too many issues open that aren't actionable will make it harder to track the actionable work.

If you think your request is an obvious improvement, open an issue. If you want to discuss a larger addition, long-term plans, or just aren't sure, start a discussion. **Just use your judgement, this isn't a big deal.** Worst case, your issue will be closed, or I'll ask you to open one.


## Making Improvements

If you want to fix a bug, add a new feature, or improve an existing one, just fork the repository, make your change, and submit a pull request. Once submitted, I will review the pull request, give feedback and possibly request changes. Once everything is in order, I'll merge it.

Pull requests are always welcome. But of course, there's a risk that yours might not be accepted. Bug fixes and other obvious improvements are usually safe, but new features might be deemed out of scope.

If you don't want to risk spending time on work that might not be merged, you can start a discussion first. [Matrix] or [Discussions] are the best ways to do that.


## Finding Work

You want to get involved, but aren't sure what to work on? We use [issues] to track actionable work that can start soon or (ideally) right now. So if you want, dig into that and get busy.

There are a lot of open issues, however. If you need some more guidance, there are some ways to narrow it down:

- Issues that are suitable for starting out are labeled as [`good first issue`](https://github.com/hannobraun/Fornjot/labels/good%20first%20issue). Those typically don't require any deep knowledge of the Fornjot code base, so they're ideal to get your feet wet.
- Some issues that need extra attention are labeled as [`help wanted`](https://github.com/hannobraun/Fornjot/labels/help%20wanted). Don't take that too seriously though. Help is welcome everywhere, not just on issues explicitly labeled as such.
- Issues are typically labeled by topic to indicate which part of the project they affect. Check out the [list of labels](https://github.com/hannobraun/Fornjot/labels), specifically the various `topic: ` labels.

Always feel free to just ask. If you have a specific issue in mind, just comment there. Or direct your query to [Matrix] or [Discussions].

If you're not a programmer, or are looking for some variety, you can also work on [the website](https://github.com/hannobraun/www.fornjot.app).


## Additional Guidelines

Let's put one thing up front: The following guidelines are just that, guidelines. **These are not mandatory rules** (except for the few that are enforced in the CI build).

If you're not sure about something or think you might forget, **don't worry.** These guidelines are here to help make the process as smooth as possible, not hinder anyone's work. Just submit a pull request and we'll figure out together what it takes to get it merged.

### Issues

Before starting to work on an issue, feel free to leave a message there first. Maybe others are working on the same issue too, or maybe there is new information that hasn't been posted in the issue yet.

### Pull Requests

#### Favor small, incremental changes

If a pull request gets too large or stays open for too long, it'll turn into a pain. Mostly for you, because you have to keep merging the latest changes from the `main` branch to stay up-to-date. But also for others, because once your pull request is merged, they'll have to deal with one big change at once, which is harder than dealing with multiple small changes over time.

If you can split a big change into a series of smaller, incremental changes, please do so! If some of those smaller changes are self-contained, like cleanup work that's required to implement your main change for example, submit those as separate pull requests, to get them merged right away.

#### Don't be afraid to change existing code

Fornjot isn't the result of some stringent top-down design process. It grows and changes, based on evolving requirements. If you see something that doesn't seem to make sense, that's probably because it actually doesn't. Maybe it once did, but then things changed around it to create the current situation.

Never be afraid to change code like that! If something is in your way, don't think you have to work around it. Just modify the existing code, until your change becomes easy.

If you're worried about making a mistake, feel free to just ask. But really, if you make a change that's wrong, and it doesn't trigger a failure in an automated test, *and* gets through review... that's not your fault at all.

#### Pull request lifecycle

If your pull request is a work in progress, please submit it as a draft to make that clear. Once you believe it is ready to be reviewed, convert the draft into a pull request.

Once your pull request has been reviewed, but not yet merged, please add any additional changes as new commits. This makes the review process much easier. If you're making changes before the pull request has been reviewed, for example in response to the CI build, feel free to modify the existing commits, if that makes things clearer.

#### Dealing with changes

If the code you're modifying has changed in the `main` branch, favor `git rebase` over `git merge`, unless there's a good reason not to. `git rebase` will lead to a linear history that is easier to understand.

#### Formatting

All Rust code follows standard formatting using [rustfmt](https://github.com/rust-lang/rustfmt), with some additional configuration in `rustfmt.toml` in the repository root. This is enforced in the CI build, so code that doesn't follow the standard formatting can't be merged.

The best way to deal with that is configuring your IDE such that it automatically formats the code every time you save the file. This often leads to sub-optimal results (compared to carefully considered manual formatting), but it saves so much mental overhead that it's more than worth it.

If you don't have it set up like that, you can run `cargo fmt` manually. Ideally, every commit should be formatted correctly, as that introduces the least friction, but having a dedicated `cargo fmt` commit from time to time is also fine.

#### Changelog

Don't worry about the changelog! It gets updates as part of the release procedure, so making changes there as part of your pull request is not necessary.

### Commits

#### Favor small, focused commits

- Focus each commit on one change. Don't combine multiple changes into the same commit.
- Don't make commits too large, unless it can't be avoided.

This makes it much easier to review your commits, as many small and focused commits are much easier to review than few large ones.

#### Each commit should compile

Each single commit should compile, without any errors or test failures, and preferably without warnings. This is a huge help when rummaging around the Git history, especially when [bisecting](https://git-scm.com/docs/git-bisect).

#### Include changes to `Cargo.lock`

When making certain changes to a `Cargo.toml` file, Cargo will automatically update the `Cargo.lock` on the next build. These updates to `Cargo.lock` should be included in the same commit with the changes that triggered them.

If you don't do this, `Cargo.lock` will be updated on the next `cargo build`/`cargo test`/..., which is going to be confusing, and can get in the way when doing a `git bisect`.

### Commit Messages

The ideal commit message consists of the following:
- An initial line of up to 50 characters.
- A blank line following it.
- Any number of additional lines, limited to 72 characters.

This is based on the [official guideline](https://git-scm.com/docs/git-commit#_discussion) and makes sure the commit is properly formatted by various tools (e.g. on GitHub or in `git log`).

Further, the initial line ideally follows these guidelines:
- Summarize the change itself and the intent behind it. This is often not possible in the limited space. Second-best is to summarize the change, or even just where it happened, and leave the intent for the rest of the message.
- Use the imperative mood, i.e. formulate the initial line like a command or request. For example, write "Add a feature" instead of "Added a feature" or "Adding a feature". This is often simplest and most compact, and therefore easiest to read.

The commit message as a whole ideally follows these guidelines:
- First and foremost, document the *intent* behind the change. Explain *why* you did something. Explaining the change itself is secondary.
- Ideally, the change itself is small and clear enough that it doesn't need explanation, or even a summary. If it does though, that should go into the commit message.
- Refrain from explaining how code you added or changed works, beyond a short summary. While such explanation is often highly beneficial, it belongs in the code itself, as a comment.
- If the intent behind a change is relevant to understanding the code after the change, then leave even that out of the commit message, and add it as a comment instead.


[issues]: https://github.com/hannobraun/Fornjot/issues
[Matrix]: https://matrix.to/#/#fornjot:pub.solar
[Discussions]: https://github.com/hannobraun/Fornjot/discussions
[@hannobraun]: https://github.com/hannobraun
