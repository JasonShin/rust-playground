#![feature(new_uninit, allocator_api)]
use std::alloc::System;

fn main() {

    let mut four = Box::<u32>::new_uninit();

    let four = unsafe {
      four.as_mut_ptr().write(2);
      four.as_mut_ptr().write(2);
      four.as_mut_ptr().write(2);
      four.assume_init()
    };

    // If you don't write anything and run `assume_init` it becomes 0 as u32 default is 0
    println!("checking four {:?}", four);

    let mut five = Box::<u32>::new_uninit();

    let five = unsafe {
        five.as_mut_ptr().write(5);
        five.assume_init()
    };

    assert_eq!(*five, 5);

    let six = Box::new(6);
    assert_eq!(*six, 6);

    let zeroed = Box::<u32>::new_zeroed();
    println!("zeroed {:?}", zeroed);
    let zeroed = unsafe { zeroed.assume_init() };
    println!("zeroed {:?}", zeroed);

    let seven = Box::new_in(7, System);
    assert_eq!(7, *seven);

    let mut eight = Box::<i32>::pin(8);
    println!("eight {:?}", *eight);
    println!("eight {:?}", *eight);

    println!("eight {:?}", *eight);
    std::mem::swap(&mut eight, &mut Box::pin(10));

    println!("eight {:?}", *eight);
}
