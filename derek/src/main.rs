use std::{ i32 };
use std::io::stdin;

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

    let mut iter = random_string.split_whitespace();
    let mut indiv_word = iter.next();

    loop {
        match indiv_word {
            Some(x) => println!("Indiv {}", x),
            None => break,
        }
        indiv_word = iter.next();
    }

    let rand_string2 = "I am a string string string string string string string string string\nhaha haha\nhey hey hey";

    let mut lines = rand_string2.lines();
    let mut indiv_line = lines.next();

    loop {
        match indiv_line {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_line = lines.next();
    }

    println!("Find best {}", rand_string2.contains("best"));

    /*
    'outer: loop {
        let number: i32 = 10;
        println!("Pick a number");

        loop {
            let mut line = String::new();
            let input = stdin().read_line(&mut line);

            let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

            match guess {
                None => println!("Enter a Number"),
                Some(n) if n == number => {
                    println!("You guessed it!");
                    break 'outer;
                }
                Some(n) if n < number =>
                println!("Too low"),
                Some(n) if n > number =>
                println!("Too low"),
                Some(_) => println!("Error")
            }
        }
    }*/

    let rand_array = [1, 2, 3, 4, 5];
    println!("{}", rand_array[0]);
    println!("{}", rand_array.len());
    println!("Second 2 : {:?}", &rand_array[1..3]);

    let mut vect1 = vec![1, 2, 3, 4];

    for i in &vect1 {
        println!("Vect: {}", i);
    }

    vect1.push(6);
    vect1.pop();

    let rand_tuple = ("Derek", 20);
    let rand_tuple2: (&str, i8) = ("Derek", 40);

    println!("Name: {}", rand_tuple2.0);

    say_hello("Jason");
    println!("5 + 4 = {}", get_sum(5, 4));

    let sum = get_sum;
    println!("6 + 4 = {}", sum(6, 4));

    // Closures
}

fn say_hello(name: &str) {
    println!("Hello {}", name);
}

fn get_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}