use bevy::{prelude::*, window::PrimaryWindow};
use num;
use crate::bullet::*;

const PLAYER_SPEED: f32 = 500.0;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_camera,
                spawn_player,
            ))
            .add_systems(Update, (
                player_movement,
                player_shoot,
            ))
            ;
    }
}

#[derive(Component)]
struct Player {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/spaceship.png"),
            transform: Transform::from_scale(Vec3::splat(0.5))
                .with_translation(Vec3::new(window.width() / 2.0, 64.0, 0.0)),
            ..default()
        },
        Player{}
    ));
}

fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

        let window = window_query.get_single().unwrap();
        transform.translation.x = num::clamp(transform.translation.x, 0.0, window.width());
    }
}

fn player_shoot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut bullet_event_writer: EventWriter<BulletShotEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let player_position = player_query.get_single().unwrap().translation;
        bullet_event_writer.send(
            BulletShotEvent{
                positon: player_position,
                direction: Vec3::Y,
            }
        );
    }
}