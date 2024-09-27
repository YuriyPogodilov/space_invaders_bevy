use bevy::{
    math::bounding::{
        Aabb2d, 
        BoundingCircle, 
        IntersectsVolume,
    }, 
    prelude::*, 
    window::PrimaryWindow,
};
use crate::{
    bullet::{
        BulletShotEvent,
        Instigator,
    },
    enemy::{
        Enemy,
        EnemyState,
        ENEMY_COLLIDER_RADIUS,
    },
};
use num;

const PLAYER_SPEED: f32 = 500.0;
const SHOOTING_COOLDOWN: f32 = 0.5;
const PLAYER_SIZE: Vec2 = Vec2::new(108.0, 64.0);
const PLAYER_COLLIDER_V_SIZE: Vec2 = Vec2::new(32.0, 62.0);
const PLAYER_COLLIDER_H_SIZE: Vec2 = Vec2::new(106.0, 18.0);
const PLAYER_COLLIDER_H_SHIFT: Vec2 = Vec2::new(0.0, -15.0);

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
                shooting_point.y += PLAYER_SIZE.y / 2.0 + 1.0;
                bullet_event_writer.send(
                    BulletShotEvent{
                        instigator: Instigator::Player,
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
    enemy_collider: Query<(Entity, &Transform, &Enemy)>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        let player_position = player_transform.translation.truncate();

        let player_box_v = Aabb2d::new(
            player_position,
            PLAYER_COLLIDER_V_SIZE / 2.0,
        );
        let player_box_h = Aabb2d::new(
            player_position + PLAYER_COLLIDER_H_SHIFT,
            PLAYER_COLLIDER_H_SIZE / 2.0,
        );

        let mut collided = false;
        for (enemy_entity, enemy_transform, enemy) in &enemy_collider {
            if enemy.state != EnemyState::Kamikaze {
                continue;
            }
            let enemy_box = BoundingCircle::new(
                enemy_transform.translation.truncate(), 
                ENEMY_COLLIDER_RADIUS
            );
            if enemy_box.intersects(&player_box_h) {
                collided = true;
            }
            if enemy_box.intersects(&player_box_v) {
                collided = true;
            }
            if collided {
                commands.entity(player_entity).despawn();
                commands.entity(enemy_entity).despawn();
                break;
            }
        }
    }
}