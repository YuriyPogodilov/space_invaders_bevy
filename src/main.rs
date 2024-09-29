mod game;
mod sound;
mod main_menu;

use bevy::{
    prelude::*, 
    window::PrimaryWindow,
    window::WindowResolution,
};
use game::{player::PlayerEvent, GamePlugin};
use sound::SoundPlugin;
use main_menu::MainMenuPlugin;

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
        .init_state::<AppState>()
        .add_plugins(SoundPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (
            transition_to_game_state,
            transition_to_main_menu,
            handle_game_over,
            exit_game,
        ))
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
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

fn transition_to_game_state(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::InGame {
            next_state.set(AppState::InGame);
        }
    }
}

fn transition_to_main_menu(
    mut next_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
        }
    }
}

fn handle_game_over(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut player_event_reader: EventReader<PlayerEvent>,
) {
    for event in player_event_reader.read() {
        match event {
            PlayerEvent::Died => next_app_state.set(AppState::GameOver),
        }
    }
}

fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}
