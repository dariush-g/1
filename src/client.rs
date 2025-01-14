mod audio;
mod block;
mod bullets;
mod colliders;
mod enemy;
mod game;
mod player;

use std::{
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream, UdpSocket},
};

use bevy::prelude::*;
use bevy::{ecs::query::With, state::commands, time, utils::HashMap};
use bincode;
use bullets::{move_bullets, shoot, Bullet, Velocity};
use colliders::colliders_o::{CircleCollider, ColliderType};
use enemy::{EnemyBarrel, EnemyPlayer};
use game::Game;

use crate::audio::audioO::BulletSoundtrackPlayer;
use player::{player_movement, Barrel, Player};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::enemy::EnemyPlayerHealthBar;
use crate::player::PlayerHealthBar;

type Players = Arc<Mutex<Vec<Player>>>;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");
    stream
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    App::new()
        .insert_resource(TcpStrRes {
            stream,
            connected: true,
        })
        .add_plugins(Game)
        .add_systems(Startup, setup)
        .add_systems(Update, network_update)
        .add_systems(Update, work_enemy)
        .run();
}

fn setup(
    mut stream: ResMut<TcpStrRes>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_id: String = generate_random_string();

    if let Err(e) = stream.stream.write_all(player_id.as_bytes()) {
        eprintln!("Couldn't send id to server: {}", e);
    }

    let player = Player {
        id: player_id, //player_id,
        position: (-1000., 0.),
        angle: 0.,
        just_shot: false,
        current_health: 1000,
    };

    Player::spawn(&mut commands, &mut meshes, &mut materials, player);
    EnemyPlayer::spawn(&mut commands, &mut meshes, &mut materials, (1000., 0.));
}
#[derive(Resource, Debug)]
struct TcpStrRes {
    stream: TcpStream,
    connected: bool,
}

fn network_update(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut stream: ResMut<TcpStrRes>,
    mut queries: ParamSet<(
        Query<(&Transform, &Player)>,
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Barrel>>,
    )>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let mut just_shot: bool = false;
    if !stream.connected {
        return;
    }

    let player_pos = queries.p1().single().translation.truncate();

    // Get the window and cursor position
    let window = windows.single();
    let cursor_position = if let Some(position) = window.cursor_position() {
        position
    } else {
        return;
    };

    if mouse.just_pressed(MouseButton::Left) {
        just_shot = true;
    }

    let (camera, camera_transform) = camera_q.single();

    let angle;

    let binding = queries.p0();
    let (transform, player_data) = binding.single();

    if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
        let direction = world_position - player_pos;
        angle = direction.y.atan2(direction.x);
    } else {
        return;
    }

    let player = Player {
        id: player_data.id.clone(),
        position: (transform.translation.x, transform.translation.y),
        angle,
        just_shot,
        current_health: player_data.current_health,
    };

    if let Ok(msg) = bincode::serialize(&player) {
        if let Err(e) = stream.stream.write_all(&msg) {
            eprintln!("Failed to write to stream: {}", e);
            stream.connected = false;
        }
    }
}

fn work_enemy(
    // mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut queries: ParamSet<(
        Query<&mut Transform, With<EnemyBarrel>>,
        Query<(&mut Transform, &EnemyPlayer)>,
        Query<&Player>,
        Query<Entity, With<EnemyPlayerHealthBar>>,
        Query<Entity, With<EnemyPlayer>>,
    )>,
    asset_server: Res<AssetServer>,
    mut stream: ResMut<TcpStrRes>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut buffer = [0; 1024];
    match stream.stream.read(&mut buffer) {
        Ok(0) => {
            println!("No enemy data yet");
        }
        Ok(msg) => {
            if let Ok(data) = bincode::deserialize::<Player>(&buffer[..msg]) {
                let player_id = if let Ok(player) = queries.p2().get_single() {
                    player.id.clone()
                } else {
                    return;
                };

                let mut binding = queries.p0();
                let mut barrel_transform = binding.single_mut();
                let angle = data.angle;
                barrel_transform.rotation = Quat::from_rotation_z(angle);
                barrel_transform.translation = Vec3::new(angle.cos() * 70., angle.sin() * 70., 1.0);

                if data.just_shot {
                    let bullet_sound = "sounds/gunshot.ogg";
                    let bsp = BulletSoundtrackPlayer::new(bullet_sound, asset_server);
                    BulletSoundtrackPlayer::play_bullet_sound(bsp, &mut commands);
                    let bullet_position = barrel_transform.translation
                        + Vec3::new(angle.cos() * 140.0, angle.sin() * 140.0, 0.0);
                    let velocity = Vec2::new(angle.cos(), angle.sin()) * 3500.0;
                    let bullet = Bullet {
                        velocity: Velocity {
                            x: velocity.x,
                            y: velocity.y,
                        },
                        position: (
                            queries.p1().single().0.translation.x + angle.cos() * 80.,
                            queries.p1().single().0.translation.y + angle.sin() * 80.,
                        ),
                    };
                    commands.spawn((
                        bullet,
                        CircleCollider {
                            collider_type: ColliderType::Bullet,
                            radius: 10.0,
                        },
                        Mesh2d(meshes.add(Circle::new(10.0))),
                        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
                        Transform::from_translation(bullet_position),
                        Velocity {
                            x: velocity.x,
                            y: velocity.y,
                        },
                    ));
                }

                let enemy_player = queries.p4().single();

                let entity = queries.p3().single_mut();

                match data.current_health {
                    800 => {
                        commands.entity(entity).despawn();
                        commands
                            .spawn((
                                EnemyPlayerHealthBar {
                                    current_health: 800.0,
                                },
                                Mesh2d(meshes.add(Rectangle::new(104., 25.))),
                                MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                                Transform::from_xyz(0., -110., 5.0),
                            ))
                            .set_parent(enemy_player);
                    }
                    600 => {
                        commands.entity(entity).despawn();

                        commands
                            .spawn((
                                EnemyPlayerHealthBar {
                                    current_health: 600.0,
                                },
                                Mesh2d(meshes.add(Rectangle::new(78., 25.))),
                                MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                                Transform::from_xyz(0., -110., 5.0),
                            ))
                            .set_parent(enemy_player);
                    }
                    400 => {
                        commands.entity(entity).despawn();

                        commands
                            .spawn((
                                EnemyPlayerHealthBar {
                                    current_health: 400.0,
                                },
                                Mesh2d(meshes.add(Rectangle::new(52., 25.))),
                                MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                                Transform::from_xyz(0., -110., 5.0),
                            ))
                            .set_parent(enemy_player);
                    }
                    200 => {
                        commands.entity(entity).despawn();

                        commands
                            .spawn((
                                EnemyPlayerHealthBar {
                                    current_health: 200.0,
                                },
                                Mesh2d(meshes.add(Rectangle::new(26., 25.))),
                                MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                                Transform::from_xyz(0., -110., 5.0),
                            ))
                            .set_parent(enemy_player);
                    }
                    0 => {
                        commands.entity(entity).despawn();
                    }
                    _ => {
                        eprintln!("Unhandled player health value: {}", data.current_health);
                    }
                }

                if data.id != player_id {
                    for (mut transform, _) in queries.p1().iter_mut() {
                        println!(
                            "{} position: ({}, {}), angle: {}, health: {}",
                            data.id,
                            data.position.0,
                            data.position.1,
                            data.angle,
                            data.current_health,
                        );

                        transform.translation.x = data.position.0;
                        transform.translation.y = data.position.1;
                    }
                }
            } else {
                eprintln!("Failed to deserialize player data");
            }
        }
        Err(e) => {
            eprintln!("Error reading from stream: {}", e);
        }
    }
}

fn generate_random_string() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = (0..6).map(|_| rng.gen_range(b'a'..=b'z') as char).collect();
    chars.into_iter().collect()
}
