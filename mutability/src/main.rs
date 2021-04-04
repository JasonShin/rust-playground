fn main() {
    let mut my_number = 2;
    println!("my number is {}", my_number);
    my_number = 1;
    println!("Hello, world! {}", my_number);
    let g = 2;
    let h = &&&g;
    let x = *h;
    println!("test test {}", x);
    let z = **h + *x;
}
