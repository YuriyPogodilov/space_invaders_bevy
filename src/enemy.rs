use  bevy::{prelude::*, window::PrimaryWindow};
use rand::seq::IteratorRandom;
use crate::player::Player;

const ENEMIES_PER_WAVE: u32 = 16;
const ENEMIES_PER_ROW: u32 = 8;
const ENEMY_SIZE: f32 = 64.0;
const ENEMY_SPEED: f32 = 200.0;
const KAMIKAZE_TIMER: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, (
                enemy_movement,
                kamikaze_attack,
                update_kamikaze_timer,
                return_to_base,
                back_to_idle,
            ))
            ;
    }
}

#[derive(Component)]
struct Enemy {
    state: EnemyState,
    base_position: Vec3,
    direction: Vec3,
}

#[derive(Eq, PartialEq)]
enum EnemyState {
    Idle,
    Kamikaze,
    ReturningToBase,
}

#[derive(Component, Deref, DerefMut)]
struct KamikazeTimer(Timer);

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let begin_x = window.width() / 2.0 - (ENEMY_SIZE * (2 * ENEMIES_PER_ROW - 1) as f32) / 2.0;
    let begin_y = window.height() - 2.0 * ENEMY_SIZE;

    for n in 0..ENEMIES_PER_WAVE {
        let row = n / ENEMIES_PER_ROW;
        let x = begin_x + 2.0 * ENEMY_SIZE * (n - ENEMIES_PER_ROW * row) as f32;
        let y = begin_y - ENEMY_SIZE * row as f32;
        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/enemy.png"),
                ..default()
            },
            Enemy{
                state: EnemyState::Idle,
                base_position: Vec3::new(x, y, 0.0),
                direction: Vec3::ZERO,
            }
        ));
    }
}

fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        if enemy.direction != Vec3::ZERO {
            transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
        }
    }
}

fn kamikaze_attack(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    cooldowns: Query<&KamikazeTimer, With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if !cooldowns.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();
    let (entity, transform, mut enemy) = enemy_query.iter_mut().choose(&mut rng).unwrap();

    if enemy.state != EnemyState::Idle {
        return;
    }

    let player_transform = player_query.get_single().unwrap();
    enemy.direction = (player_transform.translation - transform.translation).normalize();
    enemy.state = EnemyState::Kamikaze;
    commands.entity(entity).insert(KamikazeTimer(
        Timer::from_seconds(KAMIKAZE_TIMER, TimerMode::Once),
    ));
}

fn update_kamikaze_timer(
    mut commands: Commands,
    mut timers_query: Query<(Entity, &mut KamikazeTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in &mut timers_query {
        if timer.tick(time.delta()).just_finished() {
            commands.entity(entity).remove::<KamikazeTimer>();
        }
    }
}

fn return_to_base(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        if enemy.state != EnemyState::Kamikaze {
            continue;
        }
        if transform.translation.y < -100.0 {
            transform.translation.y = window.height() + 100.0;
            enemy.direction = (enemy.base_position - transform.translation).normalize();
            enemy.state = EnemyState::ReturningToBase;
        }
    }
}

fn back_to_idle(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        if enemy.state != EnemyState::ReturningToBase {
            continue;
        }

        let distance = transform.translation.distance(enemy.base_position);
        // TODO: Might not work on low fps. Needs a better solution
        if distance < 3.0 {
            transform.translation = enemy.base_position;
            enemy.direction = Vec3::ZERO;
            enemy.state = EnemyState::Idle;
        }
    }
}