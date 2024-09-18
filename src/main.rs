mod player;
mod bullet;

use bevy::prelude::*;
use bullet::BulletPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(BulletPlugin)
        .run();
}

