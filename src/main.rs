extern crate duel;
extern crate rusty_sword_arena;

use duel::{audio_loop, parse_args};
use rusty_sword_arena::VERSION;
use std::sync::mpsc::channel;
use std::thread::Builder;

fn main() {
    let (name, host) = parse_args();
    println!("Using rusty_sword_arena v{}, name: {}, host: {}",
             VERSION, name, host);

    // Run the audio system in a separate thread
    let (audio_tx, audio_rx) = channel::<u8>();
    let audio_handle = Builder::new()
        .name("Audio System".to_string())
        .spawn(move|| audio_loop(audio_rx))
        .unwrap();

    // Shut down
    let _ = audio_tx.send(0);
    let _ = audio_handle.join();
}

