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
