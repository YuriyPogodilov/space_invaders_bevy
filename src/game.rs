use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
        ;
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}