use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::HashMap, io::Write};
use tempfile::tempdir;
use std::fs;
use std::env;

struct Hello {
    path: Option<PathBuf>
}

impl Hello {
    fn new(gen_path: Option<PathBuf>) -> Hello {
        Hello {
            path: gen_path
        }
    }
}

fn lol(hello: Hello) {
    let dir = tempdir().unwrap();
    let parent_path = dir.path();
    let gen_path = hello.path;
    let gen_path_exists = &gen_path.is_some();
    if gen_path_exists == &true {
        let gen_path = gen_path.clone();
        let path_buf = &gen_path.unwrap();
        let gen_path = path_buf.as_path();
        let gen_path = parent_path.join(gen_path);
        let gen_path = gen_path.display();
        println!("gen_path is none {gen_path}");
    }

    if gen_path_exists == &true {
        let generate_path = parent_path.join(gen_path.unwrap().as_path());
        let type_file = fs::read_to_string(generate_path);
    }
}

fn main() {
    let x = PathBuf::from("gen_types");
    let z = Hello::new(None);
    lol(Hello::new(None));
}
