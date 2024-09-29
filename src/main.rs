mod player;
mod bullet;
mod enemy;
mod game;

use bevy::{prelude::*, window::WindowResolution};
use bullet::BulletPlugin;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use game::GamePlugin;

fn main() {
    let app_window = Some(Window {
        title: "SpaceInvadersBevy".into(),
        resolution: WindowResolution::new(1280.0, 720.0),
        ..default()
    });
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: app_window,
                ..default()
            }
        ))
        .add_plugins(PlayerPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(GamePlugin)
        .run();
}

