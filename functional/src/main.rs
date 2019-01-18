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
