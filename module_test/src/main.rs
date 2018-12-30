/*
mod sound {
    fn guitar() {

    }
}
*/


/*
fn main() {
    mod sound {
        mod instrument {
            mod woodwind {
                fn clarinet() {

                }
            }
        }

        mod voice {

        }
    }

    println!("Hello, world!");
    sound::instrument::woodwind
}
*/

/*
mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut Squash");
    println!("zz {}", v.name);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
*/

// Using Use to shorten path

/*
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("Called clarinet");
        }
    }
}

// use crate::sound::instrument;
use self::sound::instrument;

fn main() {
    instrument::clarinet();
}
*/

/*
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
            println!("Clarinet!!");
        }
    }
}

mod performance_group {
    use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    performance_group::clarinet_trio();
}
*/

// Idiomatic invocation

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("testing!");
        }
    }
}

use crate::sound::instrument::clarinet;

fn main() {
    clarinet();
}
