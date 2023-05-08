# grade_calculator

A simple program for calculating grades for your college courses, now in the Rust edition using a Cursive-built TUI. Use it to keep track of your course grades throughout the semester and track your progress.

## Supported systems

If your system can use Rust/Cargo, it can run this `grade_calculator`. This was tested using the current version of Rust available through `apt` on Debian (`1.63`) and Homebrew on MacOS (`1.68`).

- Debian
>`$ apt install rust-all`

- MacOS
>`$ brew install rust`

**Note for Windows users**: Although untested, the TUI uses the cross-platform `crossterm` backend and should have no trouble working directly on Windows outside of WSL, if that's how you've chosen to install Rust.

## Dependencies

This program relies on the `cursive` and `cursive-aligned-view` crates on `crates.io`.

## Installation

### From source

Open a terminal window inside the root directory of this repository. You should see the files `Cargo.toml`, `Cargo.lock`, and the `src` subdirectory. Run the following command:

>`$ cargo build --release`

The binary will be present in the `target/release` subdirectory as `grade_calculator` on MacOS and Linux or `grade_calculator.exe` on Windows.

### From crates.io

Open a terminal window and type the following command:

>`$ cargo install grade_calculator`

Cargo should give appropriate information about sourcing your shell configuration files.

# Usage

Either move or link to the binary at the path mentioned above if built from source. If downloaded from crates.io, source your shell configuration files appropriately. At a terminal prompt, type the following:

- Linux/MacOS
>`$ grade_calculator`

- Windows
>`>grade_calculator.exe`

# Documentation

To view this crate's documentation, open a terminal window at the path mentioned above and type the following:

>`$ cargo doc --open`

The documentation should open up in your default web browser.
