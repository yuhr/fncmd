<div align="center"><br><br>

<p>Command line interface as a function.</p>
<h1>fncmd</h1>

[![crates.io](https://img.shields.io/crates/v/fncmd)](https://crates.io/crates/fncmd)
[![docs.rs](https://img.shields.io/docsrs/fncmd)](https://docs.rs/fncmd/latest/fncmd/)
[![License](https://img.shields.io/github/license/yuhr/fncmd)](https://github.com/yuhr/fncmd/blob/develop/LICENSE)

<br><br></div>

`fncmd` is an opinionated command line parser frontend that wraps around [`clap`](https://crates.io/crates/clap). The functionality is mostly identical to `clap`, but provides much more automated and integrated experience.

## Motivation

Imagine a command line program you want to create. Essentially, it can be abstracted as a simple function that takes command line options as arguments. Then there should be nothing to stop you from being able to write it *literally* as a function, without using structs or builders like today's Rustaceans do.

This concept is tremendously inspired by [`argopt`](https://crates.io/crates/argopt), I really appreciate the work. However, it still requires a bit of cumbersome code, especially for handling subcommands. `fncmd` has been rewritten from scratch to get rid of all the complexities. Let's dig into [Subcommands](#subcommands) section to see how we can handle it.

## Installation

**This crate is nightly-only**. Make sure you have set up your toolchain as nightly before using (e.g. having [`rust-toolchain`](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file) file). You might be interested in [Why nightly](#why-nightly).

To install, run in your project directory:

```sh
cargo add fncmd
```

## Basics

This crate exposes just a single attribute macro, [`fncmd`], which can **only** be attached to the `main` function:

```rust
// main.rs

/// Description of the command line tool
#[fncmd::fncmd]
pub fn main(
  /// Argument foo
  #[opt(short, long)]
  foo: String,
  /// Argument bar
  #[opt(short, long)]
  bar: Option<String>,
) {
  println!("{:?} {:?}", foo, bar);
}
```

That's all, and now you got a command line program with options handled by `clap`. With above code, the help message will be like below:

```console
$ crate-name --help
Description of the command line tool

Usage: crate-name [OPTIONS] --foo <FOO>

Options:
  -f, --foo <FOO>  Argument foo
  -b, --bar <BAR>  Argument bar
  -h, --help       Print help
  -V, --version    Print version

```

The name and the version of your crate are automatically inferred from Cargo metadata.

The usage of the `opt` attribute is exactly the same as the [underlying `clap` attribute on arguments](https://github.com/clap-rs/clap#using-derive-macros), they're just passed as is, except that it appends `(long)` if no configuration was provided, i.e. `#[opt]` means `#[opt(long)]`. If you want to take the argument `foo` without `--foo`, just omit `#[opt]`.

## Subcommands

As you may know, in Cargo project [you can put entrypoints for additional binaries into `src/bin`](https://doc.rust-lang.org/cargo/guide/project-layout.html). If 1) their names are prefixed by `crate-name` and 2) their `main` functions are decorated with the `#[fncmd]` attribute and 3) exposed as `pub`, then those are automatically wrapped up as subcommands of the default binary target `crate-name`. Say you have the following directory structure:

```plaintext
src
├── main.rs
└── bin
    ├── crate-name-subcommand1.rs
    └── crate-name-subcommand2.rs
```

You'll get the following subcommand structure:

```plaintext
crate-name
├── crate-name subcommand1
└── crate-name subcommand2
```

## Multiple commands and nested subcommands

Actually `fncmd` doesn't have any distinction between the “default” binary and “additional” binaries. It determines subcommand structure just based on prefix structure instead. Therefore, configuring binary targets in your `Cargo.toml` should work as intended, for example:

```toml
[[bin]]
name = "crate-name"
path = "src/clis/crate-name.rs"

[[bin]]
name = "another"
path = "src/clis/another.rs"

[[bin]]
name = "another-sub" # `pub`
path = "src/clis/another-sub.rs"

[[bin]]
name = "another-sub-subsub" # `pub`
path = "src/clis/another-sub-subsub.rs"

[[bin]]
name = "another-orphan" # non-`pub`
path = "src/clis/another-orphan.rs"

[[bin]]
name = "another-orphan-sub" # `pub`
path = "src/clis/another-orphan-sub.rs"
```

This configuration yields up into these commands:

```plaintext
crate-name

another
└── another sub
    └── another sub subsub

another-orphan
└── another-orphan sub
```

Note that `another-orphan` is not contained within `another`, because it's not exposed as `pub`. As seen here, making the `main` of a target non-`pub` is only meaningful when you want it to have a common prefix with others but not to be included by another command, so in most cases you can set `pub` without thinking.

Of course the same structure can be achieved without manually editing `Cargo.toml`, by placing files into the default location:

```plaintext
src
├── main.rs
└── bin
    ├── another.rs
    ├── another-sub.rs
    ├── another-sub-subsub.rs
    ├── another-orphan.rs
    └── another-orphan-sub.rs
```

## Use with exotic attribute macros

Sometimes you may want to transform the `main` function with another attribute macro such as `#[tokio::main]` and `#[async_std::main]`. In such case you have to put `#[fncmd]` at the outmost level:

```rust
/// Description of the command line tool
#[fncmd]
#[tokio::main]
pub async fn main(hello: String) -> anyhow::Result<()> {
  ...
}
```

But not:

```rust
/// Description of the command line tool
#[tokio::main]
#[fncmd]
pub async fn main(hello: String) -> anyhow::Result<()> {
  ...
}
```

This is because the macros like `#[tokio::main]` do some assertions on their own, so we need to feed them a well-mannered version of `main` function, e.g. removing parameters.

Position of the doc comment doesn't matter.

## Restrictions

`fncmd` won't support following features by design. That's why `fncmd` states “opinionated”.

### Can't show authors in the help message

Showing authors in the help will simply be a noise from general user's point of view.

### Can't change the name and the version of the commands to different values

Changing metadata such as `name` and `version` to different values from the ones defined in `Cargo.toml` can easily undermine maintainability and consistency of them.

### Can't attach `#[fncmd]` to functions other than `main`

Attaching `#[fncmd]` to arbitrary functions can lead to a bloated single file codebase, which should be avoided in general.

## Why nightly

The way it automatically determines which targets are subcommands or not requires the `#[fncmd]` macro itself to know the name of the attached target, and thus the path of the file at which it has been called. This can be achieved by [`Span::source_file`](https://doc.rust-lang.org/proc_macro/struct.Span.html#method.source_file), which is behind an unstable feature flag `proc_macro_span`.