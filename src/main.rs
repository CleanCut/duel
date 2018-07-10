use std::env;

fn main() {
    parse_args();
}

fn parse_args() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: duel NAME HOST");
        std::process::exit(2);
    }
    let host = args.pop().unwrap();
    let name = args.pop().unwrap();
    println!("{} {}", name, host);
}