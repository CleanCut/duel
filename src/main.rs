use std::env;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: duel NAME HOST");
        return;
    }
    let host = args.pop().unwrap();
    let name = args.pop().unwrap();
    println!("{} {}", name, host);
}
