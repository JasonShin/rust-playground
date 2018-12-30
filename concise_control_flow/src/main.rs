#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    /*
    let mut count = 0;
    if let Coin::Quarter(UsState::Alabama) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    */
}
