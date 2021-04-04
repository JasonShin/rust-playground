fn main() {
    println!("Hello, world! C:\\Home\\JasonShin"); // Unescaped
    println!(r#"test C:\Home\JasonShin"#); // Raw string
    println!(r###"yoyo C:\Home"###);
    println!("{:?}", b"This looks like a number"); // printed
    println!("{:#?}", b"This looks like a pretty number"); // pretty printed
}
