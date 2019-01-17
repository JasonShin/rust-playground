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
    let sum_nums = |x: i32, y: i32| x + y;
    println!("{}", sum_nums(7, 8));

    let num_ten = 10;
    let add_10 = |x: i32| x + num_ten;
    println!("5 + 10 = {}", add_10(5));

    // References
    let vect1 = vec![1, 2, 3];
    let vect2 = vect1;
    // println!("vect1[0] : {}", vect1[0]); // throws an error:  value borrowed here after move...
    println!("vect1[0] : {}", vect2[0]);

    let prim_val = 1;
    let prim_val2 = prim_val;
    println!("prim_val : {}", prim_val);

    println!("Sum of vect : {}", sum_vects(&vect2));

    println!("Vect 2 : {:?}", &vect2);

    // Struct => used to define custom data types
    let mut circle1 = Circle {
        x: 10.0,
        y: 10.0,
        radius: 10.0,
    };

    println!("x {}, y {}, radius {}", circle1.x, circle1.y, circle1.radius);
    println!("Circle radius {}", get_radius(&circle1));
    println!("Circle radius {}", circle1.get_x());

    // structs
    let mut rect1 = Rectangle {
        height: 10.1,
        width: 12.2,
    };
    println!("Rect radius {}", rect1.area());

    // enum
    let hulk = Hero::Strong(1234);
    let quicksilver = Hero::Fast;
    let spiderman = Hero::Info {name: "Spiderman".to_owned(), secret: "Pter Parker".to_owned(), };

    get_info(hulk);
    get_info(spiderman);
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

enum Hero {
    Fast,
    Strong(i32),
    Info { name: String, secret: String }
}

fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("Fast!"),
        Hero::Strong(i) => println!("Lifts {}", i),
        Hero::Info {name, secret} => {
            println!("{} is {}", name, secret);
        }
    }
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {sum += x; sum});
    return sum;
}

struct Rectangle {
    height: f64,
    width: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14159 * (self.radius * self.radius)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn say_hello(name: &str) {
    println!("Hello {}", name);
}

fn get_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}