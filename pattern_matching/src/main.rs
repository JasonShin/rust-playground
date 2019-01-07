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

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    println!("Hello, world!");
    println!("Checking value of Penny {}", value_in_cents(Coin::Penny));
    println!("Checking value of Nickel {}", value_in_cents(Coin::Nickel));
    println!("Checking value of Quarter {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    plus_one(five);
    let none = plus_one(None);

    // some_u8_value should match _
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
