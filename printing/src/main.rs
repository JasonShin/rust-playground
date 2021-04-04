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

    let father_name = "Vlad";
    let son_name = "Adrian";
    let family_name = "Tepes";
    println!("This is {1} {2} son of {0} {2}", father_name, son_name, family_name);

    println!("{city1} is a city in {country} and {city2} is not a city in {country}",
        city1="Seoul",
        city2="Tokyo",
        country="Korea",
    );
}
