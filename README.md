# Migra

Simple SQL migration manager for your project.


### Install

```bash
cargo install migra
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
