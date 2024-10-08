use bevy::{
    math::bounding::{
        Aabb2d, 
        BoundingCircle, 
        IntersectsVolume,
    }, 
    prelude::*, 
    window::PrimaryWindow,
};
use crate::AppState;
use crate::game::{
    bullet::{
        Bullet,
        BulletShotEvent,
        BULLET_SIZE,
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
            .add_event::<PlayerEvent>()
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(OnExit(AppState::InGame), despawn_player)
            .add_systems(Update, (
                player_movement,
                player_shoot,
                update_cooldown,
                check_collision_with_enemy,
                check_collision_with_bullet,
                listen_player_event,
            ).run_if(in_state(AppState::InGame)))
            ;
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct ShootingCooldown(Timer);

#[derive(Event)]
pub enum PlayerEvent {
    Died,
}

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

fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
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
    player_query: Query<&Transform, With<Player>>,
    enemy_collider: Query<(Entity, &Transform, &Enemy)>,
    mut player_event_writer: EventWriter<PlayerEvent>,
) {
    if let Ok(player_transform) = player_query.get_single() {
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
                player_event_writer.send(PlayerEvent::Died);
                commands.entity(enemy_entity).despawn();
                break;
            }
        }
    }
}

fn check_collision_with_bullet(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    player_query: Query<&Transform, With<Player>>,
    mut player_event_writer: EventWriter<PlayerEvent>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_position = player_transform.translation.truncate();

        let player_box_v = Aabb2d::new(
            player_position,
            PLAYER_COLLIDER_V_SIZE / 2.0,
        );
        let player_box_h = Aabb2d::new(
            player_position + PLAYER_COLLIDER_H_SHIFT,
            PLAYER_COLLIDER_H_SIZE / 2.0,
        );

        for (bullet_entity, bullet_transforom, bullet) in &bullet_query {
            if bullet.instigator == Instigator::Player {
                continue;
            }

            let bullet_box = Aabb2d::new(
                bullet_transforom.translation.truncate(), 
                BULLET_SIZE / 2.0,
            );

            let mut collided = false;
            if bullet_box.intersects(&player_box_h) {
                collided = true;
            }
            if bullet_box.intersects(&player_box_v) {
                collided = true;
            }
            if collided {
                player_event_writer.send(PlayerEvent::Died);
                commands.entity(bullet_entity).despawn();
                break;
            }
        }
    }
}

fn listen_player_event(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut player_event_listener: EventReader<PlayerEvent>,
) {
    for event in player_event_listener.read() {
        match event {
            PlayerEvent::Died => {
                if let Ok(player_entity) = player_query.get_single() {
                    commands.entity(player_entity).despawn();
                }
            },
        }
    }
}            