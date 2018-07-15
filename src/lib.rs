//! This is the library for Duel.  Yay!

extern crate impose;
extern crate rusty_sword_arena;

pub mod audio;
pub mod player;

pub use audio::audio_loop;

/// `parse_args()` is a fancy function that parses arguments for you.  It expects the an argument
/// for name and host.
pub fn parse_args() -> (String, String) {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: duel NAME HOST");
        std::process::exit(2);
    }
    let host = args.pop().unwrap();
    let name = args.pop().unwrap();
    (name, host)
}
