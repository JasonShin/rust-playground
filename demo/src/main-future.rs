use futures::executor::block_on;

async fn dance() {
    println!("dancing");
}

async fn sing_and_song() {
    println!("sing and song");
}

async fn learn() {
    let f1 = dance();
    let f2 = sing_and_song();
    futures::join!(f1, f2);
}

fn main() {
    block_on(learn())
}
