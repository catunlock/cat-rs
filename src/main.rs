use std::clone::Clone;

#[derive(Clone, Debug)]
struct Point (i32, i32);

use std::env;
use std::process::exit;

fn main() {

    let mut my_args = Vec::new();
    let args = env::args();
    for arg in args {
        println!("User passed: {}", arg);
        my_args.push(arg);
    }

    match my_args.len() {
        1 => {
            println!("Not enough arguments supplied");
            exit(1);
        }
        _ => {
            for path in &my_args[1..] {
                let content = std::fs::read_to_string(path).unwrap();
                print!("{}", content);
            }
            
        }
    }

}
