use bevy::prelude::*;
use serde::*;

use crate::{
    audio::audioO::{BulletImpactSoundtrackPlayer, BulletSoundtrackPlayer},
    colliders::colliders_o::{CircleCollider, ColliderType},
    game::{MAP_HEIGHT, MAP_WIDTH},
    player::*,
};

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Bullet {
    #[allow(dead_code)]
    pub velocity: Velocity,
    pub position: (f32, f32),
}

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub fn shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut queries: ParamSet<(
        Query<Entity, With<Player>>,
        Query<&Transform, With<Barrel>>,
        Query<&Transform, With<Player>>,
    )>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let bullet_sound = "sounds/gunshot.ogg";
        let bsp = BulletSoundtrackPlayer::new(bullet_sound, asset_server);
        BulletSoundtrackPlayer::play_bullet_sound(bsp, &mut commands);

        if let Ok(barrel_transform) = queries.p1().get_single() {
            let angle = barrel_transform.rotation.to_euler(EulerRot::XYZ).2;
            let bullet_position = barrel_transform.translation
                + Vec3::new(angle.cos() * 140.0, angle.sin() * 140.0, 0.0);
            let velocity = Vec2::new(angle.cos(), angle.sin()) * 3500.0;
            let bullet = Bullet {
                velocity: Velocity {
                    x: velocity.x,
                    y: velocity.y,
                },
                position: (
                    queries.p2().single().translation.x + angle.cos() * 80.,
                    queries.p2().single().translation.y + angle.sin() * 80.,
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
    }
}

pub fn move_bullets(
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(Entity, &mut Transform, &mut Bullet, &Velocity)>,
        Query<&Transform, With<Player>>,
    )>,
    mut asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let bullet_sound = "sounds/impact.ogg";
    let value = BulletImpactSoundtrackPlayer::new(bullet_sound, &mut asset_server);
    let player_pos = (
        queries.p1().single().translation.x,
        queries.p1().single().translation.y,
    );

    for (entity, mut transform, mut bullet, velocity) in queries.p0().iter_mut() {
        transform.translation.x = bullet.position.0;
        transform.translation.y = bullet.position.1;

        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        bullet.position.0 = transform.translation.x;
        bullet.position.1 = transform.translation.y;

        if transform.translation.x.abs() > MAP_WIDTH / 2.0
            || transform.translation.y.abs() > MAP_HEIGHT / 2.0
        {
            let dist = get_length(player_pos, bullet.position);
            let mut volume = (dist / 800.) / 10.;
            if volume > 0.4 {
                volume = 0.25;
            }
            BulletImpactSoundtrackPlayer::play_impact_sound(&value, &mut commands, 0.4 - volume);
            commands.entity(entity).despawn();
        }
    }
}

pub fn get_length(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt()
}
