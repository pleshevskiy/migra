# Migra

[![CI](https://github.com/pleshevskiy/migra/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/pleshevskiy/migra/actions/workflows/rust.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Crates.io](https://img.shields.io/crates/v/migra-cli)](https://crates.io/crates/migra-cli)
![Crates.io](https://img.shields.io/crates/l/migra-cli)

Simple SQL migration manager for your project.

### Install

```bash
cargo install migra-cli
```

if you want to use dotenv for configure migra cli, just run the following in your terminal.

```bash
cargo install migra-cli --features dotenv
```


### Usage

A few steps to get you started

1. Initialize migra configuration (Optional)
    ```bash
    migra init
    ```
2. Make your first migration
    ```bash
    migra make initial_migration
    ```
3. Check applied and pending migrations
    ```bash
    migra ls
    ```
4. Upgrade your database
    ```bash
    migra up
    ```

For more information about the commands, simply run `migra help`

### Supported databases

- [x] Postgres
- [x] MySQL


## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE_APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE_MIT) or
   https://opensource.org/licenses/MIT)
