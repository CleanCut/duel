extern crate duel;
extern crate rusty_sword_arena;

use duel::{audio_loop, parse_args};
use rusty_sword_arena::VERSION;

fn main() {
    let (name, host) = parse_args();
    println!("Using rusty_sword_arena v{}, name: {}, host: {}",
             VERSION, name, host);
    audio_loop();
}

