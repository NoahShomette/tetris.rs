use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::{close_on_esc, WindowMode};
use rand::Rng;

pub mod board;
pub mod game_state_machine;

use crate::board::board_core;
use crate::board_core::{BlockID, BlockId, BoardData, BoardPointCoordinates, CurrentPlayerControlled, PieceType};
use crate::game_state_machine::{GameFlow, GamePlayState, GameStateInfo};

const TIME_STEP: f32 = 1.0 / 60.0;
const BLOCK_FALL_SPEED_UP: f32 = 0.1;
const FONT_ASSET_PATH: &str = ("OpenSans-Regular.ttf");

struct SpawnController {
    spawn_piece: bool,
}
impl FromWorld for SpawnController {
    fn from_world(world: &mut World) -> Self {
        SpawnController { spawn_piece: true }
    }
}

struct InputController {
    slow_hold_move_finished: bool,
    are_holding_down: bool,
    can_move: bool,
    time_between_moves_default: f32,
    time_between_moves_held_down: f32,
    time_till_next_move: f32,
}
impl FromWorld for InputController {
    fn from_world(world: &mut World) -> Self {
        InputController {
            slow_hold_move_finished: false,
            are_holding_down: false,
            can_move: true,
            time_between_moves_default: 0.2,
            time_between_moves_held_down: 0.05,
            time_till_next_move: 0.0,
        }
    }
}



struct GameSettings{
    game_randomizer: PieceRandomizerType,
    
}

enum PieceRandomizerType{
    Bag,
    TrueRandom,
    TrueRandomWithoutRepeats,
}


impl FromWorld for GameSettings {
    fn from_world(world: &mut World) -> Self {
        GameSettings {
            game_randomizer: PieceRandomizerType::Bag,
        }
    }
}

struct Randomizer{
    current_bag:Vec<PieceType>
}


impl FromWorld for Randomizer {
    fn from_world(world: &mut World) -> Self {
        Randomizer {
            current_bag: Randomizer::new_bag()
        }
    }
}

impl Randomizer {
    //creates a new standard 7 set bag
    fn new_bag() -> Vec<PieceType>{
        let new_bag= vec![PieceType::I, PieceType::J, PieceType::L,PieceType::O, PieceType::S, PieceType::T, PieceType::Z];
        new_bag
    }

    //returns a random piece type from the current bag
    fn next_block(&mut self) -> PieceType{
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.current_bag.len());
        let piece_to_return:PieceType = self.current_bag[index];
        self.current_bag.remove(index);
        if self.current_bag.len() == 0{
            self.current_bag = Randomizer::new_bag();
        }
        piece_to_return
    }
}


#[derive(Default)]
struct TickEvent {}

struct GameTickInfo {
    do_tick: bool,
    base_time_between_ticks: f32,
    time_between_ticks: f32,
    time_till_next_tick: f32,
}
impl FromWorld for GameTickInfo {
    fn from_world(world: &mut World) -> Self {
        GameTickInfo {
            do_tick: false,
            base_time_between_ticks: 0.5,
            time_between_ticks: 0.5,
            time_till_next_tick: 0.5,
        }
    }
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
        .add_event::<TickEvent>()
        .init_resource::<GameStateInfo>()
        .init_resource::<GameTickInfo>()
        .init_resource::<SpawnController>()
        .init_resource::<InputController>()
        .init_resource::<GameSettings>()
        .init_resource::<Randomizer>()
        //
        /*
        .add_system(game_start_input)
        .add_system(handle_game_input)
        .add_system(game_loop_control)
        .add_system(game_tick_manager)
*/
        
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(game_start_input)
                .with_system(handle_game_input.after(game_loop_control))
                .with_system(game_loop_control.after(game_tick_manager))
                .with_system(game_tick_manager),
        )
        .add_system(handle_game_state_events)
        //
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn game_tick_manager(
    time: Res<Time>,
    mut game_tick_time: ResMut<GameTickInfo>,
    mut event_writer: EventWriter<TickEvent>,
) {
    if game_tick_time.do_tick {
        game_tick_time.time_till_next_tick += time.delta().as_secs_f32();
        if game_tick_time.time_till_next_tick >= game_tick_time.time_between_ticks {
            game_tick_time.time_till_next_tick -= game_tick_time.time_between_ticks;
            event_writer.send(default());
        }
    }
}

fn game_start_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameStateInfo>,
    mut event_writer: EventWriter<GamePlayState>,
) {
    if game_state.game_state == GamePlayState::Menu {
        if keyboard_input.pressed(KeyCode::Space) {
            game_state.change_game_play_state(GamePlayState::Playing, event_writer);
        }
    }
}

fn handle_game_state_events(
    mut event_reader: EventReader<GamePlayState>,
    mut game_tick_time: ResMut<GameTickInfo>,
) {
    for event in event_reader.iter() {
        if *event == GamePlayState::Playing {
            game_tick_time.do_tick = true;
        }
    }
}

fn game_loop_control(
    asset_server: Res<AssetServer>,
    
    mut randomizer: ResMut<Randomizer>,
    game_settings: Res<GameSettings>,

    mut spawn_controller: ResMut<SpawnController>,
    mut game_state_reader: EventReader<GamePlayState>,
    mut tick_reader: EventReader<TickEvent>,
    mut board_data: ResMut<BoardData>,
    mut commands: Commands,
    mut blocks_query: Query<(
        Entity,
        &BlockID,
        &mut BoardPointCoordinates,
        &mut Transform,
        Option<&CurrentPlayerControlled>,
    )>,
    mut highest_block_id: ResMut<BlockId>,
) {
    for tick in tick_reader.iter() {
        if spawn_controller.spawn_piece == true {
            //info!("spawned piece");
            board_core::spawn_new_block(
                &mut commands,
                &mut board_data,

                match game_settings.game_randomizer {
                    PieceRandomizerType::Bag => randomizer.next_block(),
                    PieceRandomizerType::TrueRandom => randomizer.next_block(),
                    PieceRandomizerType::TrueRandomWithoutRepeats => randomizer.next_block(),
                },
                &asset_server,
                &mut highest_block_id,
            );
            spawn_controller.spawn_piece = false;
            board_core::move_all_pieces(&mut board_data, &mut commands, &mut blocks_query);
        } else {
            let something_moved =
                board_core::move_all_pieces(&mut board_data, &mut commands, &mut blocks_query);
            spawn_controller.spawn_piece = !something_moved;
        }

        //info!(spawn_controller.spawn_piece)
    }
}

fn handle_game_input(
    time: Res<Time>,
    mut tick_info: ResMut<GameTickInfo>,
    keyboard_input: Res<Input<KeyCode>>,
    mut input_controller: ResMut<InputController>,
    mut game_state: ResMut<GameStateInfo>,
    mut board_data: ResMut<BoardData>,
    mut commands: Commands,
    mut blocks_query: Query<
        (Entity, &BlockID, &mut BoardPointCoordinates, &mut Transform),
        With<CurrentPlayerControlled>,
    >,
) {
    if input_controller.are_holding_down {
        
        if input_controller.slow_hold_move_finished {
            input_controller.time_till_next_move += time.delta().as_secs_f32();
            if input_controller.time_till_next_move >= input_controller.time_between_moves_held_down {
                input_controller.time_till_next_move -= input_controller.time_between_moves_held_down;
                input_controller.can_move = true;
            }
        }else{
            input_controller.time_till_next_move += time.delta().as_secs_f32();
            if input_controller.time_till_next_move >= input_controller.time_between_moves_default {
                input_controller.time_till_next_move -= input_controller.time_between_moves_default;
                input_controller.can_move = true;
                input_controller.slow_hold_move_finished = true;
            }
        }
    }
    
    

    if game_state.game_state == GamePlayState::Playing && input_controller.can_move {
        if keyboard_input.pressed(KeyCode::A) && input_controller.can_move {
            board_core::move_piece_horizontal(
                &mut board_data,
                &commands,
                &mut blocks_query,
                IVec2 { x: -1, y: 0 },
            );
            input_controller.are_holding_down = true;
            input_controller.can_move = false;
        }

        if keyboard_input.pressed(KeyCode::D) && input_controller.can_move {
            board_core::move_piece_horizontal(
                &mut board_data,
                &commands,
                &mut blocks_query,
                IVec2 { x: 1, y: 0 },
            );
            input_controller.are_holding_down = true;
            input_controller.can_move = false;
        }

        
    }

    if keyboard_input.just_released(KeyCode::A) || keyboard_input.just_released(KeyCode::D) {
        input_controller.are_holding_down = false;
        input_controller.slow_hold_move_finished = false;
        input_controller.can_move = true;
        input_controller.time_till_next_move = 0.0;

    }

    if keyboard_input.pressed(KeyCode::S){
        tick_info.time_between_ticks = BLOCK_FALL_SPEED_UP;
    }
    if keyboard_input.just_released(KeyCode::S){
        tick_info.time_between_ticks = tick_info.base_time_between_ticks;
    }
}
