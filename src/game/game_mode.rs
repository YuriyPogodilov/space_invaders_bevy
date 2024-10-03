use bevy::prelude::*;
use crate::AppState;
use super::enemy::{
    Enemy,
    EnemyEvent,
};

pub struct GameModePlugin;

impl Plugin for GameModePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameModeData>()
            .add_event::<GameModeEvent>()
            .add_systems(Update, (
                listen_enemy_event,
            ).run_if(in_state(AppState::InGame)))
        ;
    }
}

#[derive(Resource, Default)]
struct GameModeData {
    score: u32,
    highest_score: u32,
    wave: u8
}

#[derive(Event)]
pub enum GameModeEvent {
    ScoreChanged(u32),
    HighestScoreChanged(u32),
    WaveChanged(u8),
}

fn listen_enemy_event(
    mut enemy_event_reader: EventReader<EnemyEvent>,
    mut game_mode_data: ResMut<GameModeData>,
    mut game_mode_event_writer: EventWriter<GameModeEvent>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for event in enemy_event_reader.read() {
        match event {
            EnemyEvent::Died(_) => {
                game_mode_data.score += 1;
                game_mode_event_writer.send(GameModeEvent::ScoreChanged(game_mode_data.score));
    
                if game_mode_data.score > game_mode_data.highest_score {
                    game_mode_data.highest_score = game_mode_data.score;
                    game_mode_event_writer.send(GameModeEvent::HighestScoreChanged(game_mode_data.highest_score));
                }

                if enemy_query.iter().count() == 1 {
                    game_mode_data.wave += 1;
                    game_mode_event_writer.send(GameModeEvent::WaveChanged(game_mode_data.wave));
                }
            }
        }
    }
}