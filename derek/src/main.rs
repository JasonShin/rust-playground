fn main() {
    let random_string = "jason shin test";
    println!("Hello, world! length {}", random_string.len());

    let (first, second) = random_string.split_at(6);
    println!("first {} second {}", first, second);

    let mut chars = random_string.chars();
    let mut indiv_char = chars.next();

    loop {
        match indiv_char {
            Some(x) => println!("char {}", x),
            None => break,
        }
        indiv_char = chars.next();
    }

}
