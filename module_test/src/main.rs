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

fn main() {
    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut Squash");
    println!("zz {}", v.name)
}
