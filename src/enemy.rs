use bevy::{prelude::*, window::PrimaryWindow};

const ENEMIES_PER_WAVE: u32 = 16;
const ENEMIES_PER_ROW: u32 = 8;
const ENEMY_SIZE: f32 = 64.0;

#[derive(Component)]
struct Enemy {}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_enemies)
            ;
    }
}

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
            Enemy{}
        ));
    }
}