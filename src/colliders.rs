pub mod colliders_o {
    use bevy::prelude::*;

    use crate::{
        audio::audioO::BulletImpactSoundtrackPlayer, bullets, colliders, enemy, player::Player,
    };

    use bullets::*;
    use colliders::*;

    #[derive(Component)]
    #[allow(warnings)]
    pub struct CircleCollider {
        pub collider_type: ColliderType,
        pub radius: f32, // Assumes circular colliders for simplicity
    }

    #[derive(Component)]
    #[allow(warnings)]
    pub struct BlockCollider {
        pub collider_type: ColliderType,
        pub points: ((f32, f32), (f32, f32), (f32, f32), (f32, f32)),
        pub width: f32,
        pub height: f32,
    }

    #[allow(warnings)]
    pub enum ColliderType {
        Enemy,
        Player,
        Block,
        Bullet,
    }

    pub struct CollidersPlugin;

    impl Plugin for CollidersPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Update, detect_block_and_circle_collision)
                .add_systems(Update, detect_block_and_bullet_collision);
        }
    }

    pub fn detect_block_and_circle_collision(
        mut player_q: Query<&mut Transform, (With<CircleCollider>, Without<Bullet>)>,
        block_q: Query<(Entity, &BlockCollider)>,
    ) {
        let mut player = player_q.single_mut();
        let player_x = player.translation.x;
        let player_y = player.translation.y;
        let player_offset = 70.0;
        let buffer = 2.0; // Small buffer to prevent sticking

        for (_entity, block) in block_q.iter() {
            let block_x_min = block.points.0 .0;
            let block_x_max = block.points.2 .0;
            let block_y_max = block.points.0 .1;
            let block_y_min = block.points.2 .1;

            // Early exit if not near block
            if player_x + player_offset < block_x_min - player_offset
                || player_x - player_offset > block_x_max + player_offset
                || player_y + player_offset < block_y_min - player_offset
                || player_y - player_offset > block_y_max + player_offset
            {
                continue;
            }

            let closest_x = player_x.clamp(block_x_min, block_x_max);
            let closest_y = player_y.clamp(block_y_min, block_y_max);

            let dx = player_x - closest_x;
            let dy = player_y - closest_y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < player_offset + buffer {
                if distance > 0.1 {
                    let target_distance = player_offset + buffer;
                    let scale = target_distance / distance;

                    player.translation.x = closest_x + dx * scale;
                    player.translation.y = closest_y + dy * scale;
                } else {
                    player.translation.x += player_offset;
                }
            }
        }
    }

    // pub fn detect_block_and_circle_collision(
    //     mut player_q: Query<&mut Transform, (With<CircleCollider>, Without<Bullet>)>,
    //     block_q: Query<(Entity, &BlockCollider)>,
    // ) {
    //     let mut player = player_q.single_mut();

    //     let player_x = player.translation.x;
    //     let player_y = player.translation.y;

    //     let player_offset = 70.0;

    //     for (_entity, block) in block_q.iter() {
    //         let block_x_min = block.points.0 .0;
    //         let block_x_max = block.points.2 .0;
    //         let block_y_max = block.points.0 .1;
    //         let block_y_min = block.points.2 .1;

    //         if (player_x + player_offset > block_x_min
    //             && player_x + player_offset < block_x_max
    //             && player_y > block_y_min
    //             && player_y < block_y_max)
    //             || (player_x - player_offset > block_x_min
    //                 && player_x - player_offset < block_x_max
    //                 && player_y > block_y_min
    //                 && player_y < block_y_max)
    //             || (player_x > block_x_min
    //                 && player_x < block_x_max
    //                 && player_y - player_offset > block_y_min
    //                 && player_y - player_offset < block_y_max)
    //             || (player_x > block_x_min
    //                 && player_x < block_x_max
    //                 && player_y + player_offset > block_y_min
    //                 && player_y + player_offset < block_y_max)
    //         {
    //             if player_x + player_offset > block_x_min && player_x < block_x_min {
    //                 player.translation.x = block_x_min - player_offset;
    //             } else if player_x - player_offset < block_x_max && player_x > block_x_max {
    //                 player.translation.x = block_x_max + player_offset;
    //             }

    //             if player_y + player_offset > block_y_min && player_y < block_y_min {
    //                 player.translation.y = block_y_min - player_offset;
    //             } else if player_y - player_offset < block_y_max && player_y > block_y_max {
    //                 player.translation.y = block_y_max + player_offset;
    //             }

    //             println!(
    //                 "Collision resolved: Player moved to ({}, {})",
    //                 player.translation.x, player.translation.y
    //             );
    //         }
    //     }
    // }

    pub fn detect_block_and_bullet_collision(
        mut commands: Commands,
        mut queries: ParamSet<(
            Query<(Entity, &mut Transform), (With<CircleCollider>, With<Bullet>, Without<Player>)>,
            Query<&Transform, With<Player>>,
        )>,
        mut asset_server: Res<AssetServer>,
        block_q: Query<(Entity, &BlockCollider)>,
    ) {
        let bullet_sound = "sounds/impact.ogg";
        let value = BulletImpactSoundtrackPlayer::new(bullet_sound, &mut asset_server);

        let player_pos = (
            queries.p1().single().translation.x,
            queries.p1().single().translation.y,
        );

        for (entity, mut _bullet) in queries.p0().iter_mut() {
            let bullet_x = _bullet.translation.x;
            let bullet_y = _bullet.translation.y;

            let bullet_radius = 5.0;

            for (_block_entity, block) in block_q.iter() {
                let block_x_min = block.points.0 .0;
                let block_x_max = block.points.2 .0;
                let block_y_max = block.points.0 .1;
                let block_y_min = block.points.2 .1;

                if bullet_x + bullet_radius > block_x_min
                    && bullet_x - bullet_radius < block_x_max
                    && bullet_y + bullet_radius > block_y_min
                    && bullet_y - bullet_radius < block_y_max
                {
                    let dist = get_length(player_pos, (bullet_x, bullet_y));
                    let mut volume = (dist / 800.) / 10.;

                    if volume > 0.4 {
                        volume = 0.25;
                    }

                    BulletImpactSoundtrackPlayer::play_impact_sound(
                        &value,
                        &mut commands,
                        0.4 - volume,
                    );
                    commands.entity(entity).despawn();
                    break;
                }
            }
        }
    }

    pub fn detect_player_and_bullet_collision(
        mut queries: ParamSet<(
            Query<&Transform, (With<CircleCollider>, With<Player>, Without<Bullet>)>,
            Query<
                (Entity, &Transform, &Bullet),
                (With<CircleCollider>, With<Bullet>, Without<Player>),
            >,
        )>,
        mut asset_server: Res<AssetServer>,
        mut commands: Commands,
        mut player_q: Query<&mut Player>,
    ) {
        // Get player position
        let player_pos = match queries.p0().get_single() {
            Ok(transform) => transform.translation,
            Err(_) => {
                eprintln!("Player entity not found");
                return;
            }
        };

        // Get player component
        let mut player = match player_q.get_single_mut() {
            Ok(player) => player,
            Err(e) => {
                eprintln!("Player component not found: {}", e);
                return;
            }
        };

        let player_x = player_pos.x;
        let player_y = player_pos.y;

        // Collect entities to despawn
        let mut entities_to_despawn = Vec::new();

        // Check for collisions
        for (entity, transform, _bullet) in queries.p1().iter_mut() {
            let bullet_x = transform.translation.x;
            let bullet_y = transform.translation.y;

            let dx = bullet_x - player_x;
            let dy = bullet_y - player_y;
            let distance = (dx * dx + dy * dy).sqrt();
            let collision_distance = 70.0;

            if distance < collision_distance {
                if player.current_health > 0 {
                    player.current_health -= 200
                } else {
                    println!("Player: {} is dead", player.id)
                }
                let bullet_sound = "sounds/impact.ogg";
                let value = BulletImpactSoundtrackPlayer::new(bullet_sound, &mut asset_server);
                entities_to_despawn.push(entity);
                BulletImpactSoundtrackPlayer::play_impact_sound(&value, &mut commands, 0.3);
            }
        }

        // Despawn entities after iteration
        for entity in entities_to_despawn {
            commands.entity(entity).despawn();
        }
    }
}
