const MY_NUMBER: i32 = 2;
static SEASONS: [&str; 4] = ["spring", "summer", "autumn", "winter"]; // every array has a different type / size

fn main() {
    println!("Hello, world! {}", MY_NUMBER);
    println!("test printing static {:?}", SEASONS);
    println!("first season {}", SEASONS[0]);
}
