use futures::*;
use futures::executor::block_on;

async fn hello_world() {
    println!("testing!");
}

fn main() {
    let future = hello_world();
    let future2 = (async {
        println!("async print!");
    });
    futures::executor::block_on(future)
}
