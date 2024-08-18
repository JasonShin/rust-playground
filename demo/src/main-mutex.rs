use tokio::{ sync::Mutex };

#[tokio::main]
async fn main() {
    let mut x = Mutex::new(1);
    
    let mut y = x.lock().await;
    *y = 2;
    println!("{:?} - {:?}", x, y);
    drop(y);
    let mut y = x.lock().await;
    *y = 3;
    println!("{:?}", x);
}
