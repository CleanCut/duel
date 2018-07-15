extern crate duel;
extern crate rusty_sword_arena;

use duel::{audio_loop, parse_args};
use duel::player::Player;
use rusty_sword_arena::VERSION;
use rusty_sword_arena::game::{
    ButtonState, ButtonValue, InputEvent, PlayerInput, PlayerState, Vector2
};
use rusty_sword_arena::gfx::Window;
use rusty_sword_arena::net::ServerConnection;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread::Builder;
use std::time::{Duration, Instant};

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
    let mut my_input = PlayerInput::new();
    my_input.id = my_id;
    let mut mouse_pos = Vector2::new();
    let mut window = Window::new(None);
    let mut last_input_sent = Instant::now();
    let send_input_duration = Duration::from_millis(15);

    'gameloop: loop {
        // Accumulate user input into a PlayerInput
        for input_event in window.poll_input_events() {
            match input_event {
                InputEvent::WindowClosed => break 'gameloop,
                InputEvent::MouseMoved { position } => mouse_pos = position,
                InputEvent::Button { button_value, button_state } => {
                    let amount = match button_state {
                        ButtonState::Pressed => 1.0,
                        ButtonState::Released => 0.0,
                    };
                    match button_value {
                        ButtonValue::Up => my_input.move_amount.y = amount,
                        ButtonValue::Down => my_input.move_amount.y = -amount,
                        ButtonValue::Left => my_input.move_amount.x = -amount,
                        ButtonValue::Right => my_input.move_amount.x = amount,
                        ButtonValue::Attack => {
                            my_input.attack = match button_state {
                                ButtonState::Pressed => true,
                                ButtonState::Released => false,
                            };
                        },
                        ButtonValue::Quit => break 'gameloop,
                    }
                }
            }
        }
        if let Some(my_player) = players.get(&my_id) {
            my_input.direction = my_player.state.pos.angle_between(mouse_pos);
        }


        // Periodically send accumulated input
        if last_input_sent.elapsed() > send_input_duration {
            server_conn.send_player_input(my_input.clone());
            last_input_sent = Instant::now();
        }

        for game_state in server_conn.poll_game_states() {
            // Remove any players who are no longer in the game
            players.retain(|k, _v| game_state.player_states.contains_key(k));
            // Update or add all players that have states
            for (id, player_state) in game_state.player_states {
                if players.contains_key(&id) {
                    players.get_mut(&id).unwrap().update_state(player_state);
                } else {
                    players.insert(id, Player::new(player_state, &window));
                }
            }
        }

        // Draw a frame!
        window.drawstart();
        // Draw all the bodies!
        for (_id, player) in &players {
            if player.state.dead {
                continue;
            }
            window.draw(&player.body);
        }
        // Draw all the swords!
        for (_id, player) in &players {
            if player.state.dead {
                continue;
            }
            window.draw(&player.sword);
        }
        window.drawfinish();
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
