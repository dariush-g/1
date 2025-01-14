use bevy::prelude::*;

use crate::colliders::colliders_o::{self, ColliderType};

#[derive(Component)]

pub struct Block {
    #[allow(dead_code)]
    pub points: ((f32, f32), (f32, f32), (f32, f32), (f32, f32)),
}

#[allow(warnings)]
impl Block {
    pub fn get_block_center(
        points: ((f32, f32), (f32, f32), (f32, f32), (f32, f32)),
    ) -> (f32, f32) {
        let centerx = (points.0 .0 + points.2 .0) / 2.;
        let centery = (points.0 .1 + points.2 .1) / 2.;
        return (centerx, centery);
    }

    pub fn spawn_block(
        block: Block,
        mut commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        // Create a rectangle mesh
        let rectangle = Rectangle::new(
            block.points.2 .0 - block.points.0 .0,
            block.points.3 .1 - block.points.1 .1,
        );

        let center = Self::get_block_center(block.points);

        commands.spawn((
            Block {
                points: block.points,
            },
            colliders_o::BlockCollider {
                points: block.points,
                collider_type: ColliderType::Block,
                width: block.points.2 .0 - block.points.0 .0,
                height: block.points.3 .1 - block.points.1 .1,
            },
            Mesh2d(meshes.add(rectangle)),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.5, 0.5, 0.5)))),
            Transform::from_xyz(center.0, center.1, 1.0),
        ));
    }
}
