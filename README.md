# ds3os-loader-rs
Another ds3os loader

## Usage

This program only patch memory of the running process, so you need launch game from steam first.

You may need some dev packages in debian-based distro or [you cannot launch](https://github.com/clague/ds3os-loader-rs/issues/1), for example: `libexpat-dev libfreetype-dev libssl-dev`.

## Build

First you need to install rust compiler, you can set up compile environment by using the [rustup](https://rustup.rs/).

Then clone this repo in your file system and run

`cargo build --release`
