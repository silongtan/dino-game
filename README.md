[![Build binary release](https://github.com/noahgift/rdedupe/actions/workflows/release.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/release.yml)
[![Rustfmt](https://github.com/noahgift/rdedupe/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/rustfmt.yml)

# Space Invader Game

A Rust based space invader game, using Bevy game engine.

## Description

The game features a spaceship that can move left and right, and shoot bullets. The spaceship is controlled by the left and right arrow keys. The spacebar is used to shoot bullets. The game also features a number of enemies that move down the screen. The enemies are destroyed when they are hit by a bullet. The game ends when the spaceship is hit by an enemy. Player could repawn after a short time period.

## Installation

You'll need to have Rust installed on your machine. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install

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


Once Rust is installed, clone this repository and navigate to the project directory in your terminal. Then, run the following command to build the timer:
``` cargo build --release ```

## Usage

To run the game, run the following command from the project directory:
``` cargo run --release ```


### Future Improvements

Containerize
Deploy

## Game Screenshot
<img width="600" alt="demo" src="https://user-images.githubusercontent.com/70916756/222190373-0faf085b-fb7d-4c29-ac45-27253408fe88.png">


## Reference
[bevy](https://bevyengine.org)\
[bevy guide](https://bevy-cheatbook.github.io)

