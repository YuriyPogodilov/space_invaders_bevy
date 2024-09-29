use bevy::prelude::*;

use crate::{
    bullet::{
        BulletShotEvent, 
        Instigator
    }, 
    enemy::EnemyEvent, 
    player::PlayerEvent,
};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (
                listen_player_event,
                listen_enemy_event,
                listen_bullet_event,
            ))
        ;
    }
}

#[derive(Resource, Default)]
struct SoundEffects {
    explosion_enemy: Handle<AudioSource>,
    explosion_player: Handle<AudioSource>,
    laser_shoot_enemy: Handle<AudioSource>,
    laser_shoot_player: Handle<AudioSource>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sound_effects = SoundEffects {
        explosion_enemy: asset_server.load("sounds/explosion_enemy.wav"),
        explosion_player: asset_server.load("sounds/explosion_player.wav"),
        laser_shoot_enemy: asset_server.load("sounds/laser_shoot_enemy.wav"),
        laser_shoot_player: asset_server.load("sounds/laser_shoot_player.wav"),
    };

    commands.insert_resource(sound_effects);
}

fn listen_player_event(
    mut commands: Commands,
    mut player_event_listener: EventReader<PlayerEvent>,
    sounds: Res<SoundEffects>,
) {
    for event in player_event_listener.read() {
        match event {
            PlayerEvent::Died => {
                commands.spawn(AudioBundle{
                    source: sounds.explosion_player.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            },
        }
    }
}

fn listen_enemy_event(
    mut commands: Commands,
    mut enemy_event_listener: EventReader<EnemyEvent>,
    sounds: Res<SoundEffects>,
) {
    for event in enemy_event_listener.read() {
        match event {
            EnemyEvent::Died(_) => {
                commands.spawn(AudioBundle {
                    source: sounds.explosion_enemy.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}

fn listen_bullet_event(
    mut commands: Commands,
    mut bullet_event_listener: EventReader<BulletShotEvent>,
    sounds: Res<SoundEffects>,
) {
    for event in bullet_event_listener.read() {
        let sound = match event.instigator {
            Instigator::Enemy => sounds.laser_shoot_enemy.clone(),
            Instigator::Player => sounds.laser_shoot_player.clone(),
        };
        commands.spawn(AudioBundle {
            source: sound,
            settings: PlaybackSettings::DESPAWN,
        });
    }
}