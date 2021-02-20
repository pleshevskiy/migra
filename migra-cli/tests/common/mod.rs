#![allow(dead_code)]
pub use assert_cmd::prelude::*;
pub use predicates::str::contains;
pub use std::process::Command;

pub type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

pub const ROOT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/");

pub fn path_to_file(file_name: &'static str) -> String {
    ROOT_PATH.to_owned() + file_name
}

pub const DATABASE_URL_DEFAULT_ENV_NAME: &str = "DATABASE_URL";
pub const DATABASE_URL_ENV_VALUE: &str = "postgres://postgres:postgres@localhost:6000/migra_tests";

pub struct Env {
    key: &'static str,
}

impl Env {
    pub fn new(key: &'static str, value: &'static str) -> Self {
        std::env::set_var(key, value);
        Env { key }
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        std::env::remove_var(self.key);
    }
}
