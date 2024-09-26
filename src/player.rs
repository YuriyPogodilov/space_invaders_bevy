use bevy::{
    math::bounding::{
        Aabb2d, 
        BoundingCircle, 
        IntersectsVolume
    }, 
    prelude::*, 
    window::PrimaryWindow
};
use crate::{
    bullet::*, 
    enemy::EnemyCollider
};
use num;

const PLAYER_SPEED: f32 = 500.0;
const SHOOTING_COOLDOWN: f32 = 0.5;

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
                update_cooldown,
                check_collision_with_enemy,
            ))
            ;
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct ShootingCooldown(Timer);

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/spaceship.png"),
            transform: Transform::from_xyz(window.width() / 2.0, 64.0, 0.0),
            ..default()
        },
        Player,
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
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    cooldowns: Query<&ShootingCooldown, With<Player>>,
    mut bullet_event_writer: EventWriter<BulletShotEvent>,
) {
    if let Ok((player, player_transform)) = player_query.get_single() {
        if keyboard_input.pressed(KeyCode::Space) {
            if let Err(_) = cooldowns.get(player) {
                commands
                    .entity(player)
                    .insert(
                        ShootingCooldown(
                            Timer::from_seconds(SHOOTING_COOLDOWN,TimerMode::Once)
                        )
                    );
                let mut shooting_point = Vec3::from(player_transform.translation).truncate();
                shooting_point.y += 32.0; // half of spaceship sprite
                bullet_event_writer.send(
                    BulletShotEvent{
                        positon: shooting_point,
                        direction: Vec2::Y,
                    }
                );
            }
        }
    }
}

fn update_cooldown(
    mut commands: Commands,
    mut cooldowns: Query<(Entity, &mut ShootingCooldown)>,
    time: Res<Time>,
) {
    for (entity, mut cooldown) in &mut cooldowns {
        cooldown.tick(time.delta());
        if cooldown.finished() { 
            commands.entity(entity).remove::<ShootingCooldown>();
        }
    }
}

fn check_collision_with_enemy(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_collider: Query<&Transform, With<EnemyCollider>>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        let player_position = player_transform.translation.truncate();

        let player_box_v = Aabb2d::new(
            player_position,
            Vec2::new(16.0, 31.0),
        );
        let player_box_h = Aabb2d::new(
            Vec2::new(player_position.x, player_position.y - 15.0),
            Vec2::new(53.0, 9.0),
        );

        let mut collided = false;
        for enemy_transform in &enemy_collider {
            let enemy_box = BoundingCircle::new(enemy_transform.translation.truncate(), 25.0);
            if enemy_box.intersects(&player_box_h) {
                collided = true;
                break;
            }
            if enemy_box.intersects(&player_box_v) {
                collided = true;
                break;
            }
        }

        if collided {
            commands.entity(player_entity).despawn();
        }
    }
}