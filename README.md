[![Tests](https://github.com/noahgift/rdedupe/actions/workflows/tests.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/tests.yml)
[![Build binary release](https://github.com/noahgift/rdedupe/actions/workflows/release.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/release.yml)
[![Clippy](https://github.com/noahgift/rdedupe/actions/workflows/lint.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/lint.yml)
[![Rustfmt](https://github.com/noahgift/rdedupe/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/rustfmt.yml)

# Space Invader Game

A Rust based space invader game, using Bevy game engine.

## Installation

You'll need to have Rust installed on your machine. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install

Once Rust is installed, clone this repository and navigate to the project directory in your terminal. Then, run the following command to build the timer:
``` cargo build --release ```

## Usage

To use the game, run the following command from the project directory:
``` cargo run --release ```



#### Future Improvements

Implementation
Containerize
Deploy

### Building and Running

* Build:  cd into rdedupe and run `make all`
* Run:  `cargo run -- dedupe --path tests --pattern .txt`
* Run tests:  `make test`

### OS X Install

* Install rust via [rustup](https://rustup.rs/)
* Add to `~/.cargo/config`

```bash
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```
* run `make all` in rdedupe directory
