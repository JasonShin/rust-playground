use std::fmt::Display;

macro_rules! cm {
    ($f:ident . $g:ident) => (|x| x.$g().$f())
}

fn main() {
    let add5 = |x| x + 5;
    let result: i32 = add5(4);
    println!("Hello, world!, {}", result);

    let result = adder(1)(2);
    println!("box! {}", result);

    let result = adder2(1)(2);
    println!("lambda {}", result);

    let x = apply_to(10, &adder2(5));
    println!("lambda x {}", x);

    let x: usize = apply_to("asdasg", &|x| x.len());
    println!("lambda getting string size {}", x);

    // Struct is like an object

    // generic struct
    let car = Car { company: "Hyundai", model: "Sonata", year: 1950 };
    println!("hyundai! {:?}", car);

    let l1 = vec![1, 2, 3, 4];
    let l3: Vec<i32> = l1.iter()
      .map(cm!(abs . neg))
      .collect();
}

fn adder(x: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |y| { x + y })
}

fn adder2(x: i32) ->  impl Fn(i32) -> i32 {
    move | y | { x + y }
}

fn apply_to<T, S>(x: T, fun: &Fn(T) -> S) -> S {
    fun(x)
}

fn double_me(x: i64) -> i64 {
    x + x
}

enum Shape {
    Circle(f32, f32, f32),
    Rectangle(f32, f32, f32, f32)
}

fn surface(shape: &Shape) -> f32 {
    match *shape {
        Shape::Circle(_, _, r) =>
            std::f32::consts::PI * r * r,

        Shape::Rectangle(x1, y1, x2, y2) =>
            (x2 - x1).abs() * (y2 - y1).abs()
    }
}

// Struct === object

/*
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_by(&self, dx: i32, dy: i32) -> Point {
        return Point { x: self.x + dx, y: self.y + dy }
        // return Self { x: self.x + dx, y: self.y + dy }
        // return self.x + dx;
    }
}
*/

// Mutable struct === expect the object itself to mutate

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_by_mut(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

// Generic struct
#[derive(Debug)]
struct Car<T, S, V> {
    company: T,
    model: S,
    year: V,
}

impl<T, S, V> Car<T, S, V> {
    fn show(&self) -> String
        where T: Display,
              S: Display,
              V: Display
    {
        format!("{} was produced by {} in {}",
                self.model, self.company, self.year)
    }
}

#[derive(Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

/*
impl Eq for &Colour {
    fn eq(&self, other: &Colour) -> bool {
        match (self, other) {
            (&Colour::Red, &Colour::Red) => true,
            (&Colour::Blue, &Colour::Blue) => true,
            (&Colour::Green, &Colour::Green) => true,
            (_) => false,
        }
    }
}
*/

