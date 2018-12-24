fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false!");
    }
    println!("Hello, world!");

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    /* let number2 = if condition {
        5
    } else {
        "six"
    };

    println!("zz, {}", number2);*/
    
    let mut counter = 0;

    let result2 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result2, 20);

    let a = [10, 20, 30, 40, 50];
    
    // While
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
    
    // For loop
    
    for element in a.iter() {
        println!("the value is: {}", element);
    }

}
