#[allow(warnings)]
pub mod audioO {
    use bevy::prelude::*;
    use bevy::{asset::Handle, audio::AudioSource};

    #[derive(Resource)]
    pub struct SoundtrackPlayer {
        pub track: Handle<AudioSource>,
    }

    impl SoundtrackPlayer {
        pub fn new(track: &str, asset_server: Res<AssetServer>) -> Self {
            SoundtrackPlayer {
                track: asset_server.load::<AudioSource>(track),
            }
        }
        pub fn play_track(soundtrack_player: SoundtrackPlayer, mut commands: Commands) {
            commands.spawn((
                AudioPlayer(soundtrack_player.track.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Loop,
                    volume: bevy::audio::Volume::new(0.2),
                    ..default()
                },
            ));
        }
    }

    #[derive(Resource)]
    pub struct BulletSoundtrackPlayer {
        pub sound: Handle<AudioSource>,
    }

    impl BulletSoundtrackPlayer {
        pub fn new(sound: &str, asset_server: Res<AssetServer>) -> Self {
            BulletSoundtrackPlayer {
                sound: asset_server.load::<AudioSource>(sound),
            }
        }
        pub fn play_bullet_sound(
            soundtrack_player: BulletSoundtrackPlayer,
            mut commands: &mut Commands,
        ) {
            commands.spawn((
                AudioPlayer(soundtrack_player.sound.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Once,
                    volume: bevy::audio::Volume::new(0.3),
                    ..default()
                },
            ));
        }
    }

    #[derive(Resource)]
    pub struct BulletImpactSoundtrackPlayer {
        pub sound: Handle<AudioSource>,
    }

    impl BulletImpactSoundtrackPlayer {
        pub fn new(sound: &str, asset_server: &mut Res<AssetServer>) -> Self {
            BulletImpactSoundtrackPlayer {
                sound: asset_server.load::<AudioSource>(sound),
            }
        }
        pub fn play_impact_sound(
            soundtrack_player: &BulletImpactSoundtrackPlayer,
            mut commands: &mut Commands,
            volume: f32,
        ) {
            commands.spawn((
                AudioPlayer(soundtrack_player.sound.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Once,
                    volume: bevy::audio::Volume::new(volume),
                    ..default()
                },
            ));
        }
    }
}
