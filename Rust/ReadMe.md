# Project Euler Rust Solutions
![Code Coverage Badge](wiki/coverage_badge.svg)

My [Rust](https://www.rust-lang.org/)-based solutions to [Project Euler](https://projecteuler.net/archives) problems.

## Running a Solution
Use the `cargo run` command with the `--bin` flag with a zero-padded problem number. The example below shows how to run the first solution.
```shell
cargo run --bin 001
```

## Running Tests
Use the `cargo test` command to run all library and solution tests.
```shell
cargo test
```

## Crates Used
- [clap](https://github.com/clap-rs/clap) - Command-line interface argument parser.

## To Dos
- In reference to `cards/hand.rs`; it is mostly geared towards poker which can be broken out into a separate trait to clean up the `partial_cmp` method, to stop it from arbitrarily using the `poker_hand` method. This will allow for better reuse in any other solutions that may need playing card hands.
    - If `cards/hand.rs` no longer exists and/or this has been addressed remove these lines.