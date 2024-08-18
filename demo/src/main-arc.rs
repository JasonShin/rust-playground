use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum Child {
    Toast(i32),
    Master(i32),
}

#[derive(Debug)]
struct Stuff {
    pub huh: HashMap<String, Rc<Child>>
}

impl Stuff {
    fn new() -> Stuff {
        Stuff {
            huh: HashMap::new()
        }
    }

    fn get_something(&mut self, key: &str) -> Rc<Child> {

        if let Some(z) = self.huh.get(key) {
            return z.clone();
        }

        &self.huh.insert(key.to_string(), Rc::new(Child::Toast(1)));
        let x = self.huh.get(key).unwrap();
        x.clone()
    }
}

fn main() {
    let mut stuff = Stuff::new();
    &stuff.get_something("jason");
    println!("Hello, world!");

    dbg!(&stuff);

    &stuff.get_something("jason2");
    &stuff.get_something("jason3");
    dbg!(&stuff);

    let something = Arc::new(Mutex::new(1));
    let something2 = Arc::clone(&something);
    let mut something3 = something2.lock().unwrap();
    *something3 += 1;
}
