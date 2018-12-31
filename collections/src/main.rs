fn main() {
    // Constructing vectors
    let mut v: Vec<i32> = Vec::new();
    let z = vec![1, 2, 3];

    // Updating
    v.push(5);
    v.push(6);

    // Accessing values
    let c = vec![1, 2, 3, 4, 5];
    let third: &i32 = &c[2];
    println!("The third element {}", third);

    match c.get(2) {
        Some(third) => println!("The element is {}", third),
        None => println!("There is no third element"),
    }

    // Iterating over vectors
    let b = vec![100, 32, 57];
    for i in &b {
        println!("{}", i);
    }

    let mut n = vec![100, 32, 57];
    for i in &mut n {
        *i += 50;
    }

    // Storing multiple types into a vector using enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
