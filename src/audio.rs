use impose::Audio;
use std::sync::mpsc::Receiver;

pub fn audio_loop(audio_rx : Receiver<&'static str>) {
    println!("Audio system initialized.");
    let mut audio_system = Audio::new();
    audio_system.add_audio("change_weapon", "media/change_weapon.ogg");
    audio_system.add_audio("die", "media/die.ogg");
    audio_system.add_audio("hit", "media/hit.ogg");
    audio_system.add_audio("join", "media/join.ogg");
    audio_system.add_audio("leave", "media/leave.ogg");
    audio_system.add_audio("miss", "media/miss.ogg");
    audio_system.add_audio("ow", "media/ow.ogg");
    audio_system.add_audio("spawn", "media/spawn.ogg");
    audio_system.add_audio("startup", "media/startup.ogg");
    loop {
        let choice = audio_rx.recv().unwrap();
        // Is it time to quit?
        if choice == "quit" {
            break;
        }
        audio_system.play(choice);
    }
    println!("Audio system shutdown");
}
