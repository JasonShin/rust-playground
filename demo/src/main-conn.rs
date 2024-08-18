use serde;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::HashMap, io::Write};
use tempfile::tempdir;
use std::fs;
use std::env;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestJson {
    #[serde(rename = "CONNECTION_POOL_SIZE", default = "default_pool_size")]
    pub connection_pool_size: i32,
}

fn default_pool_size() -> i32 {
    10
}

fn main() {
    let test = "\"test\"".to_string();
    let test = format!("{}", test.trim_start());
    println!("{}", test);
}
