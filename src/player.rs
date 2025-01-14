use bevy::{math::*, prelude::*};
use serde::*;
use tungstenite::http::header::AGE;

use crate::{
    colliders::colliders_o::{self, ColliderType},
    game::{MAP_HEIGHT, MAP_WIDTH},
};

// #[derive(Serialize, Deserialize, Debug, Resource, Clone)]
// pub struct NetworkPosition(pub Option<(f32, f32)>);

// impl NetworkPosition {
//     pub fn get_position(&self) -> Option<(f32, f32)> {
//         self.0
//     }
// }
#[derive(Component)]
#[allow(warnings)]
pub struct PlayerHealthBar {
    pub current_health: f32,
}

pub fn update_player_health(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut queries: ParamSet<(
        Query<Entity, With<PlayerHealthBar>>,
        Query<&Player>,
        Query<Entity, With<Player>>,
    )>,
) {
    let binding = queries.p1();
    let player_data = binding.single();
    let player_health = player_data.current_health;
    let player = queries.p2().single();

    let entity = queries.p0().single_mut();
    match player_health {
        800 => {
            commands.entity(entity).despawn();

            commands
                .spawn((
                    PlayerHealthBar {
                        current_health: 800.0,
                    },
                    Mesh2d(meshes.add(Rectangle::new(104., 25.))),
                    MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                    Transform::from_xyz(0., -110., 5.0),
                ))
                .set_parent(player);
        }
        600 => {
            commands.entity(entity).despawn();

            commands
                .spawn((
                    PlayerHealthBar {
                        current_health: 600.0,
                    },
                    Mesh2d(meshes.add(Rectangle::new(78., 25.))),
                    MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                    Transform::from_xyz(0., -110., 5.0),
                ))
                .set_parent(player);
        }
        400 => {
            commands.entity(entity).despawn();

            commands
                .spawn((
                    PlayerHealthBar {
                        current_health: 400.0,
                    },
                    Mesh2d(meshes.add(Rectangle::new(52., 25.))),
                    MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                    Transform::from_xyz(0., -110., 5.0),
                ))
                .set_parent(player);
        }
        200 => {
            commands.entity(entity).despawn();

            commands
                .spawn((
                    PlayerHealthBar {
                        current_health: 200.0,
                    },
                    Mesh2d(meshes.add(Rectangle::new(26., 25.))),
                    MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                    Transform::from_xyz(0., -110., 5.0),
                ))
                .set_parent(player);
        }
        0 => {
            commands.entity(entity).despawn();
        }
        _ => {
            eprintln!("Unhandled player health value: {}", player_health);
        }
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: String,
    pub position: (f32, f32),
    pub angle: f32,
    pub just_shot: bool,
    pub current_health: i32,
}

impl Player {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        player: Player,
    ) {
        let clone = player.clone();

        let playe: Entity = commands
            .spawn((
                player,
                colliders_o::CircleCollider {
                    collider_type: ColliderType::Player,
                    radius: 70.0,
                },
                Mesh2d(meshes.add(Circle::new(70.))),
                MeshMaterial2d(materials.add(Color::hsla(223., 0.54, 0.34, 1.))),
                Transform::from_xyz(clone.position.0, clone.position.1, 4.0),
            ))
            .id();

        commands.spawn(Camera2d).set_parent(playe);

        commands
            .spawn((
                Barrel,
                Mesh2d(meshes.add(Rectangle::new(80., 60.))),
                MeshMaterial2d(materials.add(Color::hsla(223., 0.54, 0.34, 1.))),
                Transform::from_xyz(70.0, 0.0, 4.0),
            ))
            .set_parent(playe);

        commands
            .spawn((
                PlayerHealthBar {
                    current_health: 1000.0,
                },
                Mesh2d(meshes.add(Rectangle::new(130., 25.))),
                MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                Transform::from_xyz(0., -110., 5.0),
            ))
            .set_parent(playe);

        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(135., 30.))),
                MeshMaterial2d(materials.add(Color::hsl(159., 0.92, 0.10))),
                Transform::from_xyz(0., -110., 4.0),
            ))
            .set_parent(playe);
    }
}

#[derive(Component)]
pub struct Barrel;

pub fn player_movement(
    mut query: Query<(&mut Transform, &Player)>,
    //mut network_pos: ResMut<NetworkPosition>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let speed = 800.0;

        // Get current position
        let current_x = transform.translation.x;
        let current_y = transform.translation.y;

        // Calculate boundaries (accounting for half the screen size)
        let max_x = (MAP_WIDTH / 2.0) - 60.0;
        let max_y = (MAP_HEIGHT / 2.0) - 60.0;

        // Check movement input
        if keyboard.pressed(KeyCode::KeyW) && current_y < max_y {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) && current_y > -max_y {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) && current_x > -max_x {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) && current_x < max_x {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            let new_pos = transform.translation + direction * speed * time.delta_secs();

            // Clamp the position to screen bounds
            transform.translation.x = new_pos.x.clamp(-max_x, max_x);
            transform.translation.y = new_pos.y.clamp(-max_y, max_y);
        }

        // Update the network position
        //network_pos.0 = Some((transform.translation.x, transform.translation.y));
    }
}

pub fn rotate_barrel(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Barrel>>,
        Query<&mut Player>,
    )>,
) {
    // Get player position first and store it
    let player_pos = queries.p0().single().translation.truncate();

    // Get the window and cursor position
    let window = windows.single();
    let cursor_position = if let Some(position) = window.cursor_position() {
        position
    } else {
        return;
    };

    // Get camera transform
    let (camera, camera_transform) = camera_q.single();

    let angle = 0.;
    // Convert cursor position to world coordinates
    if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
        for mut barrel_transform in queries.p1().iter_mut() {
            let direction = world_position - player_pos;
            let angle = direction.y.atan2(direction.x);

            barrel_transform.rotation = Quat::from_rotation_z(angle);
            barrel_transform.translation = Vec3::new(angle.cos() * 70., angle.sin() * 70., 0.0);
        }
        queries.p2().single_mut().angle = angle;
    }
}
