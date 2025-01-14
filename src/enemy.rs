use crate::{
    colliders::colliders_o::{self, ColliderType},
    player::{Barrel, Player, PlayerHealthBar},
};
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct EnemyPlayer {
    pub position: (f32, f32),
}

#[derive(Component)]
pub struct EnemyPlayerHealthBar {
    pub current_health: f32,
}

#[derive(Component)]
pub struct EnemyBarrel {
    angle: f32,
}

impl EnemyPlayer {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        position: (f32, f32),
    ) {
        let player = EnemyPlayer { position };
        let clone = player.clone();

        let player = commands
            .spawn((
                player,
                Mesh2d(meshes.add(Circle::new(70.))),
                MeshMaterial2d(materials.add(Color::hsla(0., 1., 0.27, 1.))),
                Transform::from_xyz(clone.position.0, clone.position.1, 2.0),
            ))
            .id();

        commands
            .spawn((
                EnemyBarrel { angle: 0.0 },
                Mesh2d(meshes.add(Rectangle::new(80., 60.))),
                MeshMaterial2d(materials.add(Color::hsla(0., 1., 0.27, 1.))),
                Transform::from_xyz(70.0, 0.0, 2.0),
            ))
            .set_parent(player);

        commands
            .spawn((
                EnemyPlayerHealthBar {
                    current_health: 100.0,
                },
                Mesh2d(meshes.add(Rectangle::new(130., 25.))),
                MeshMaterial2d(materials.add(Color::srgb(0., 100., 0.))),
                Transform::from_xyz(0., -110., 5.0),
            ))
            .set_parent(player);

        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(135., 30.))),
                MeshMaterial2d(materials.add(Color::hsl(159., 0.92, 0.10))),
                Transform::from_xyz(0., -110., 4.0),
            ))
            .set_parent(player);
    }
}


pub fn rotate_enemy_barrel(
    query: Query<&mut Transform, With<EnemyBarrel>>,
    barrel: Query<(Entity, &EnemyBarrel)>,
) {
    let mut binding = query;
    let mut barrel_transform = binding.single_mut();
    let (_, enemy_barrel) = barrel.single();
    let angle = enemy_barrel.angle;
    barrel_transform.rotation = Quat::from_rotation_z(angle);
    barrel_transform.translation = Vec3::new(angle.cos() * 70., angle.sin() * 70., 0.0);
}
