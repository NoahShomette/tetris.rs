use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowMode};

pub mod board;
pub mod game_state_machine;

use crate::board::board_core;
use crate::game_state_machine::{GamePlayState, GameStateInfo};

const FONT_ASSET_PATH: &str = ("OpenSans-Regular.ttf");

struct GameTickInfo {
    time_between_ticks: f32,
    time_till_next_tick: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: "tetris.rs".to_string(),
            width: 1920.,
            height: 1080.,
            mode: WindowMode::Windowed,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(board_core::BoardPlugin)
        .add_system(close_on_esc)
        //
        .add_event::<GamePlayState>()
        .init_resource::<GameStateInfo>()
        .add_system(game_start_input)
        .add_system(game_loop_control)
        //
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn game_loop_control(time: Res<Time>) {}

fn game_start_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameStateInfo>,
    mut event_writer: EventWriter<GamePlayState>,
) {
    
    if game_state.game_state == GamePlayState::Menu{
        if keyboard_input.pressed(KeyCode::Space) {
            game_state.change_game_play_state(GamePlayState::Playing, event_writer);
        }
    }

}
