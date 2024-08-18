use std::result::Result;

use anyhow::Error;

fn main() {
    let test: Result<Option<i32>, Error> = Err(Error::msg("test"));

    let z = test
        .map(|z| {
            println!("checking z {:#?}", z);
        })
        .map_err(|err| {
            println!("hmm error");
        });
    
}
