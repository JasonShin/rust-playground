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
    }
}
