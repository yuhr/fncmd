<div align="center">
  <br><br>
  <p>Command line interface as a function.</p>
  <h1>fncmd</h1>

  ![Crates.io](https://img.shields.io/crates/v/fncmd)
  ![Crates.io](https://img.shields.io/crates/l/fncmd)

  <br><br>
</div>

`fncmd` is an opinionated command line parser frontend that wraps around [`clap`](https://crates.io/crates/clap). The functionality is mostly identical to `clap`, but provides much more automated and integrated experience.

## Motivation

Imagine a command line program you want to create. Essentially, it can be abstracted as a simple function that takes command line options as arguments. Then there should be nothing to stop you from being able to write it *literally* as a function, without using structs or builders like today's Rustaceans do.

This concept is tremendously inspired by [`argopt`](https://crates.io/crates/argopt), I really appreciate the work. However, it still requires a bit of cumbersome code, especially for handling subcommands. `fncmd` has been rewritten from scratch to get rid of all the complexities. Let's dig into [Subcommands](#subcommands) section to see how we can handle it.

## Installation

**This crate is nightly-only**. Make sure you have set up your toolchain as nightly before using (e.g. having [`rust-toolchain`](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file) file).

To install, if you use [`cargo-edit`](https://crates.io/crates/cargo-edit):

```sh
cargo add fncmd
```

Or you can manually edit `Cargo.toml`:

```toml
[dependencies]
fncmd = "1.0.0"
```

## Basics

 This crate exposes just a single attribute macro, [`fncmd`], which can **only** be attached to the `main` function:

```rust
// main.rs
use fncmd::fncmd;

/// Description of the command line tool
#[fncmd]
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

```plaintext
crate-name 0.1.0

Description of the command line tool

USAGE:
    crate-name [OPTIONS] --foo <FOO>

OPTIONS:
    -b, --bar <BAR>    Argument bar
    -f, --foo <FOO>    Argument foo
    -h, --help         Print help information
    -V, --version      Print version information
```

The name and the version of your crate are automatically inferred from Cargo metadata.

The usage of the `opt` attribute is exactly the same as the [underlying `clap` attribute on arguments](https://github.com/clap-rs/clap#using-derive-macros), they're just passed as is, except that it appends `(long)` if no configuration was provided, i.e. `#[opt]` means `#[opt(long)]`. If you want to take the argument `foo` without `--foo`, just omit `#[opt]`.

## Subcommands

As you may know, in Cargo project you can put entrypoints for additional binaries into `src/bin`. If 1) their names are prefixed by `crate-name` and 2) their `main` functions are decorated with the `#[fncmd]` attribute and 3) exposed as `pub`, then those are automatically wrapped up as subcommands of the default binary target `crate-name`. Say you have the following directory structure:

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
└── crate-name subcommand1
└── crate-name subcommand2
```

## Specifying entrypoint paths manually

Configuring binary targets in your `Cargo.toml` should work as usual, for example:

```toml
[[bin]]
name = "crate-name"
path = "src/clis/crate-name.rs"

[[bin]]
name = "crate-name-subcommand1"
path = "src/clis/crate-name-subcommand1.rs"

[[bin]]
name = "crate-name-subcommand2"
path = "src/clis/crate-name-subcommand2.rs"
```

The resulting subcommand structure of this configuration is equivalent to what you get in the [Subcommands](#subcommands) section above. If you want a binary target to be handled by `fncmd` but not to be a subcommand regardless of its target name, just omit `pub`. So for example, when `crate-name-subcommand2` is not exposed as `pub`, it won't contained within `crate-name`.

## Nested subcommands

Following is how `#[fncmd]` macro determines which targets are subcommands (roughly explained):

1. Get the name of the call-site target itself
2. Enumerate all possible targets (`#[fncmd]`-annotated entrypoints)
3. Filter out inappropriate items (ones not prefixed by the name of the call-site target)
4. Filter out inappropriate items (ones prefixed by any other target name)
5. Filter out inappropriate items (non-`pub` targets)

These steps are done for each macroexpansion. So for example:

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

## Restrictions

`fncmd` won't support following features by design:

- Show authors on the help message
- Change the name and the version of the command to arbitrary values
- Attach `#[fncmd]` to functions other than `main`

That's why `fncmd` states “opinionated”. Showing authors on the help will simply be a noise from general user's point of view, and changing metadata such as `name` and `version` to different values from the ones defined in `Cargo.toml` can easily undermine maintainability and consistency of them. Attaching `#[fncmd]` to arbitrary functions can lead to a bloated single file codebase, which should be avoided in general.