fn main() {
    println!("Hello, world!");
    another_function(123);
    
    fn five() -> i32 { 5 };

    let x = five();
    println!("The value {}", x);
}

fn another_function(x: i32) {
    println!("Another function. {}", x);
}

