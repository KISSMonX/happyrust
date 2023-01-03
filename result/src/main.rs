use core::panic;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("/dev/null");

    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind(){
            ErrorKind::NotFound => match File::create("result rust demo.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error craeting file: {:?}", e),
            },
           other_err =>panic!("Error opening file: {:?}", other_err),
        }
    };

    println!("Hello, world!");
}
