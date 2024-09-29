pub mod player;
pub mod bullet;
pub mod enemy;

use bevy::prelude::*;
use bullet::BulletPlugin;
use player::PlayerPlugin;
use enemy::EnemyPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayerPlugin)
            .add_plugins(BulletPlugin)
            .add_plugins(EnemyPlugin)
        ;
    }
}