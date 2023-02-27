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


fn gaussian(mean: f64, variance: f64) -> impl Fn(f64) -> f64 {
    let std_dev = variance.sqrt();
    move |x| {
        (1.0 / (std_dev * 2.0 * std::f64::consts::PI).sqrt()) * (-1.0 * (x - mean).powi(2) / (2.0 * variance)).exp()
    }
} 