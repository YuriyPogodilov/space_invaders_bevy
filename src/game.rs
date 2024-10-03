pub mod player;
pub mod bullet;
pub mod enemy;
mod hud;
mod game_mode;

use bevy::prelude::*;
use bullet::BulletPlugin;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use hud::HUDPlugin;
use game_mode::GameModePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(GameModePlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(BulletPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(HUDPlugin)
        ;
    }
}