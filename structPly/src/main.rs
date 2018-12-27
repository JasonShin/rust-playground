
fn main() {

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    /*
    let user1 = User {
        username: String::from("Jason shin"),
        email: String::from("visualbabsic@gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    */

    let mut user1 = User {
        username: String::from("Jason shin"),
        email: String::from("visualbabsic@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    //user1.email = ( String.from("zz@gmail.com") );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };


    struct Colour(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
}
