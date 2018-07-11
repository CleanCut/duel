use std::sync::mpsc::Receiver;

pub fn audio_loop(audio_rx : Receiver<u8>) {
    println!("Audio system initialized.");
    loop {
        let choice = audio_rx.recv().unwrap();
        // Is it time to quit?
        if choice == 0 {
            break;
        }
    }
    println!("Audio system shutdown");
}
