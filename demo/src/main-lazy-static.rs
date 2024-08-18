use lazy_static::lazy_static;
use std::{collections::HashMap, borrow::BorrowMut};
use tokio;

lazy_static! {
    pub static ref SomeMap: HashMap<String, SomeStruct<'static>> = {
        let mut m = HashMap::new();
        m.insert("jason".to_string(), SomeStruct { name: "jason".to_string(), age: &1 });
        m
    };
}

#[derive(Debug)]
pub struct SomeStruct<'a> {
    pub name: String,
    pub age: &'a i32,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
   println!("checking main {:?}", SomeMap.get("jason"));
   // SomeMap.borrow_mut().insert("shin".to_string(), SomeStruct { name: "shin".to_string(), age: &2 });
}
