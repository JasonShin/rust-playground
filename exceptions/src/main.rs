use std::io;
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    // let f = File::open("hello.txt");

    /* let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file {:?}", error);
        },
    };
    println!("zz {:?}", f); */
    // Matching on different errors
    /* let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create a file but there was problem {:?}", e),
            },
            other_error => panic!("There was a problem opening up the file {:?}", other_error)
        }
    }; */

    // Using unwrap
    // let f = File::open("hello.txt").unwrap();

    // Using expect: similar to unwrap but lets you choose a panic!
    // let f = File::open("hello.txt").expect("Failed to open hello.txt!");

    // Propagating errors -> longer way
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        };
    }

}
