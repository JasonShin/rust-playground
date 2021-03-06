/*
fn main() {
    let width1 = 30;
    let height1 = 32;
    println!(
        "Hello, world! {}",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

/*
fn main() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectable is {}",
        area(rect1)
    )

}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/

/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 32, height: 90 };
    println!(
        "the area is {}",
        area(&rect1)
    );

    println!(
        "Displaying {:?}",
        rect1
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!(
        "The area of rectangle is {}",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
