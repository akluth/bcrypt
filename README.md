## bcrypt

bcrypt is a small commandline utility for creating [bcrypt](https://en.wikipedia.org/wiki/Bcrypt)
hashes. It is built around the [bcrypt](https://crates.io/crates/bcrypt) crate and written in Rust.

This project targets the current stable Rust toolchain and uses Rust 2024 edition.

### Usage

Simple:

    bcrypt STRING_TO_CALCULATE_HASH_FROM

This will create your hash with a default cost of 12. To specify the cost (has to be between
4 and 31) specify it via the `--cost` parameter:

    bcrypt --cost 4 STRING_TO_CALCULATE_HASH_FROM

Invalid costs are rejected by the command line parser.

### License

This project is licensed under the MIT license. See the LICENSE file in this repository
for more information.
