mod audio;
mod block;
mod bullets;
mod colliders;
mod enemy;
mod game;
mod player;

use player::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Players = Arc<Mutex<HashMap<String, Player>>>;
type ClientStreams = Arc<Mutex<HashMap<String, TcpStream>>>;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    let players: Players = Arc::new(Mutex::new(HashMap::new()));
    let client_streams: ClientStreams = Arc::new(Mutex::new(HashMap::new()));

    println!("Server listening");

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0; 512];
            let n = stream.read(&mut buffer)?;
            let iden = String::from_utf8_lossy(&buffer[..n]).to_string();

            println!("Player {} connected", iden);

            let players_clone = Arc::clone(&players);
            let client_streams_clone = Arc::clone(&client_streams);

            // Add the new player and their stream to the shared data structures
            players_clone.lock().unwrap().insert(
                iden.clone(),
                Player {
                    id: iden.clone(),
                    position: (0., 0.),
                    angle: 0.,
                    just_shot: false,
                    current_health: 1000,
                },
            );

            client_streams_clone
                .lock()
                .unwrap()
                .insert(iden.clone(), stream.try_clone().unwrap());

            // Spawn a thread to handle the client
            thread::spawn(move || handle_client(stream, players_clone, client_streams_clone, iden));
        }
    }

    Ok(())
}

fn handle_client(
    mut player_stream: TcpStream,
    players: Players,
    client_streams: ClientStreams,
    player_id: String,
) {
    let mut buffer = [0; 512];

    loop {
        match player_stream.read(&mut buffer) {
            Ok(0) => {
                println!("Player {} disconnected", player_id);
                break;
            }
            Ok(bytes_read) => {
                if let Ok(player_update) = bincode::deserialize::<Player>(&buffer[..bytes_read]) {
                    println!(
                        "Player {} moved to: ({}, {}), angle: {}, health: {}",
                        player_update.id,
                        player_update.position.0,
                        player_update.position.1,
                        player_update.angle,
                        player_update.current_health,
                    );

                    players
                        .lock()
                        .unwrap()
                        .insert(player_update.id.clone(), player_update.clone());

                    let client_streams_guard = client_streams.lock().unwrap();
                    for (id, stream) in client_streams_guard.iter() {
                        if *id != player_id {
                            if let Ok(serialized_data) = bincode::serialize(&player_update) {
                                if let Err(e) =
                                    stream.try_clone().unwrap().write_all(&serialized_data)
                                {
                                    eprintln!("Failed to send update to client {}: {}", id, e);
                                }
                            }
                        }
                    }
                } else {
                    eprintln!("Could not deserialize player info");
                }
            }
            Err(e) => {
                eprintln!("Error receiving player info: {}", e);
                break;
            }
        }
    }

    players.lock().unwrap().remove(&player_id);
    client_streams.lock().unwrap().remove(&player_id);
}
