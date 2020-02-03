extern crate librimu;

use std::env;
use std::fs;

use librimu::parser;

fn main() {
    let args : Vec<String> = env::args().collect();

    let input = fs::read_to_string(&args[1]).unwrap();

    println!("== Input ==\n{}\n==============\n\n\n", input);

    match parser::run(&input) {
        Ok((rem, token_lines)) => {
            for tokens in token_lines {
                println!("{:?}", tokens);
            }

            println!();
            println!();

            println!("== Unparsed ==\n{}\n===========", rem);
        },
        Err(e) => {
            println!("Error!");
            match e {
                nom::Err::Error((_, kind)) => println!("Kind: {:?}, {}", kind, kind.description()),
                _ => println!("ERR {:?}", e)
            }
        }
    }
}
