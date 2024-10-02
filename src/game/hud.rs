use bevy::{
    color::palettes::css::YELLOW, 
    prelude::*,
};

use crate::AppState;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), spawn_hud)
            .add_systems(OnExit(AppState::InGame), despawn_hud)
        ;
    }
}

#[derive(Component)]
struct GameHUD;

#[derive(Component)]
struct ScoreLabel;

#[derive(Component)]
struct HighScoreLabel;

#[derive(Component)]
struct WaveLabel;

fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_bundle_style = Style {
        margin: UiRect::all(Val::Px(20.0)),
        ..default()
    };
    let text_style = TextStyle {
        font: asset_server.load("fonts/digital-7.ttf"),
        font_size: 24.0,
        color: YELLOW.into()
    };

    commands.spawn(NodeBundle{
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Start,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section("Score: 00", text_style.clone())
                .with_style(text_bundle_style.clone()),
            ScoreLabel
        ));
        parent.spawn((
            TextBundle::from_section("Wave: 00", text_style.clone())
                .with_style(text_bundle_style.clone()),
            WaveLabel
        ));
        parent.spawn((
            TextBundle::from_section("Highest: 00", text_style.clone())
                .with_style(text_bundle_style.clone()),
            HighScoreLabel
        ));
    });
}

fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<GameHUD>>,
) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}