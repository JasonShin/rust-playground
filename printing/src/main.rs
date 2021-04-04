fn main() {
    println!("Hello, world! C:\\Home\\JasonShin"); // Unescaped
    println!(r#"test C:\Home\JasonShin"#); // Raw string
    println!(r###"yoyo C:\Home"###);
    println!("{:?}", b"This looks like a number"); // printed
    println!("{:#?}", b"This looks like a pretty number"); // pretty printed

    println!("{:?}", '행' as u32);
    println!("\u{D589}");

    // print out memory address
    let p = &3;
    let r = &p;
    let r2 = &&p;
    let r3 = &&&p;
    let r4 = &&&&p;
    println!("{:p}", p);
    println!("{:p}", r);
    println!("{:p}", r2);
    println!("{:p}", r3);
    println!("{:p}", r4);
}
