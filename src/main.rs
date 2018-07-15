extern crate duel;
extern crate rusty_sword_arena;

use duel::{audio_loop, parse_args};
use rusty_sword_arena::VERSION;
use rusty_sword_arena::game::PlayerState;
use rusty_sword_arena::net::ServerConnection;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread::Builder;

fn main() {
    let (name, host) = parse_args();
    println!("Using rusty_sword_arena v{}, name: {}, host: {}",
             VERSION, name, host);

    // Run the audio system in a separate thread
    let (audio_tx, audio_rx) = channel::<&'static str>();
    let audio_handle = Builder::new()
        .name("Audio System".to_string())
        .spawn(move|| audio_loop(audio_rx))
        .unwrap();
    let _ = audio_tx.send("startup");

    // Establish the network connection to the server
    let mut server_conn = ServerConnection::new(&host);
    let my_id = server_conn.join(&name);
    if my_id == 0 {
        println!("Either name is taken or server is full. Give it another try.");
        std::process::exit(3);
    }
    let game_setting = server_conn.get_game_setting();
    println!("Client v{}, Server v{}", VERSION, game_setting.version);
    if game_setting.version != VERSION {
        println!("WARNING!!!! Client and server versions don't match!");
        // std::process::exit(4);
    }

    // Everything else we need before the game loop
    let mut players : HashMap<u8, Player> = HashMap::new();

    'gameloop: loop {
        for game_state in server_conn.poll_game_states() {
            // Remove any players who are no longer in the game
            players.retain(|k, _v| game_state.player_states.contains_key(k));
            // Update or add all players that have states
            for (id, player_state) in game_state.player_states {
                if players.contains_key(&id) {
                    players.get_mut(&id).unwrap().update_state(player_state);
                } else {
                    players.insert(id, Player::new(player_state));
                }
            }
            println!("{}", players.len());
        }
    }

    // Shut down
    let _ = audio_tx.send("quit");
    let _ = audio_handle.join();
    if server_conn.leave(my_id) {
        println!("Server acknowledges leaving.");
    } else {
        println!("Server must have already kicked us.");
    }
}

#[derive(Debug)]
struct Player {
    state: PlayerState,
}

impl Player {
    fn new(state: PlayerState) -> Self {
        Self {
            state,
        }
    }
    fn update_state(&mut self, state: PlayerState) {
        self.state = state;
    }
}
