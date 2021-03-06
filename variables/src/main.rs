fn main() {
    let mut x = 5;
    println!("Hello, world! {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // string len
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces len {}", spaces);

    // data types
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0;
    let y: f32 = 3.0;

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Arrays
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a [1];
}
