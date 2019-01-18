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