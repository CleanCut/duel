use std::sync::mpsc::Receiver;

pub fn audio_loop(audio_rx : Receiver<&'static str>) {
    println!("Audio system initialized.");
    loop {
        let choice = audio_rx.recv().unwrap();
        println!("Playing audio: {}", choice);
        // Is it time to quit?
        if choice == "quit" {
            break;
        }
    }
    println!("Audio system shutdown");
}
