extern crate duel;
extern crate rusty_sword_arena;

use duel::audio_loop;
use rusty_sword_arena::VERSION;
use std::env;

fn main() {
    parse_args();
    audio_loop();
}

fn parse_args() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: duel NAME HOST");
        std::process::exit(2);
    }
    let host = args.pop().unwrap();
    let name = args.pop().unwrap();
    println!("Using rusty_sword_arena v{}, name: {}, host: {}",
             VERSION, name, host);
}
