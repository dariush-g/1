use bevy::state::commands;
use bevy::window::WindowMode;
use bevy::{math::*, prelude::*, window::WindowResolution};

use crate::audio::audioO::*;
use crate::colliders::colliders_o::*;
use crate::enemy::EnemyPlayer;
use crate::{block, bullets, colliders, enemy, player};
use block::*;
use bullets::*;
use colliders::colliders_o;
use player::*;

pub struct UpdatesPlugin;

impl Plugin for UpdatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player::rotate_barrel,
                player::player_movement.before(detect_block_and_circle_collision),
                bullets::shoot,
                bullets::move_bullets
                    .after(shoot)
                    .before(detect_block_and_bullet_collision),
                colliders_o::detect_block_and_bullet_collision.after(move_bullets),
                colliders_o::detect_block_and_circle_collision.after(player_movement),
                colliders_o::detect_player_and_bullet_collision,
                enemy::rotate_enemy_barrel,
                player::update_player_health,
            ),
        );
    }
}

pub const SCREEN_HEIGHT: f32 = 1600.0;
pub const SCREEN_WIDTH: f32 = 2560.0;
pub const MAP_HEIGHT: f32 = 20000.;
pub const MAP_WIDTH: f32 = 20000.;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        // Player::new(&mut commands, &mut meshes, &mut materials, (-1000., 0.));
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    //mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT)
                        .with_scale_factor_override(1.0),
                    title: String::from(""),
                    resizable: true,
                    ..Default::default()
                }),
                ..default()
            }),
        )
        .add_plugins(UpdatesPlugin)
        // .add_plugins(DefaultPlugins)
        // .add_plugins(FrameTimeDiagnosticsPlugin)
        // .add_plugins(EntityCountDiagnosticsPlugin::default())
        // .add_plugins(SystemInformationDiagnosticsPlugin)
        // .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup);
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    Block::spawn_block(
        Block {
            points: (
                (-5000., -4900.),
                (5000., -4900.),
                (5000., -5000.),
                (-5000., -5000.),
            ),
        },
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    Block::spawn_block(
        Block {
            points: (
                (-5000., 5000.),
                (5000., 5000.),
                (5000., 4900.),
                (-5000., 4900.),
            ),
        },
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    Block::spawn_block(
        Block {
            points: (
                (-5000., 5000.),
                (-4900., 5000.),
                (-4900., -5000.),
                (-5000., -5000.),
            ),
        },
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    Block::spawn_block(
        Block {
            points: (
                (5000., 4900.),
                (5000., 5000.),
                (5000., -5000.),
                (4900., -5000.),
            ),
        },
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    // let health = EHealth::new(100.);
    // let enemy = Enemy::new(health);
    // Enemy::spawn(enemy, &mut commands, &mut meshes, &mut materials);

    Block::spawn_block(
        Block {
            points: ((100., 500.), (1000., 500.), (1000., 100.), (100., 100.)),
        },
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    let track = "sounds/backmusic.ogg";
    let stp = SoundtrackPlayer::new(track, asset_server);
    SoundtrackPlayer::play_track(stp, commands);
}
