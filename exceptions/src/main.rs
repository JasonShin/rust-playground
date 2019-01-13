use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    /* let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file {:?}", error);
        },
    };
    println!("zz {:?}", f); */
    // Matching on different errors
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create a file but there was problem {:?}", e),
            },
            other_error => panic!("There was a problem opening up the file {:?}", other_error)
        }
    };
}
