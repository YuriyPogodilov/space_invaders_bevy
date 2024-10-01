use bevy::{
    color::palettes::css::RED, 
    prelude::*
};
use crate::AppState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_screen)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_screen)
            .add_systems(Update, (
                restart_action,
                restart_button_update,
            ).run_if(in_state(AppState::GameOver)))
        ;
    }
}

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
struct GameOverScreen;

#[derive(Component)]
enum GameOverActions{
    Restart,
    Quit,
}

fn spawn_game_over_screen(
    mut commands: Commands,
) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(100.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR.into(),
        ..default()
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        GameOverScreen,
    )).with_children(|parent| {
        parent
            .spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "GAME OVER", 
                        TextStyle{
                            font_size: 80.0,
                            color: RED.into(),
                            ..default()
                        }));
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            ..default()
                        },
                        GameOverActions::Restart,
                    )).with_children(|parent| {
                        parent.spawn(TextBundle::from_section("Restart", text_style.clone()));
                    });
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            ..default()
                        },
                        GameOverActions::Quit,
                    )).with_children(|parent| {
                        parent.spawn(TextBundle::from_section("Quit", text_style.clone()));
                    });
                });
    });
}

fn despawn_game_over_screen(
    mut commands: Commands,
    query: Query<Entity, With<GameOverScreen>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn restart_action(
    interaction_query: Query<(&Interaction, &GameOverActions), (Changed<Interaction>, With<GameOverActions>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                GameOverActions::Restart => next_app_state.set(AppState::InGame),
                GameOverActions::Quit => {
                    app_exit_event_writer.send(AppExit::Success);
                }
            }
        }
    }
}

fn restart_button_update(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<GameOverActions>)>,
) {
    for (interaction, mut bg_color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Hovered {
            *bg_color = HOVERED_BUTTON.into();
        } else {
            *bg_color = NORMAL_BUTTON.into();
        }
    }
}