use bevy::prelude::*;
use bevy::window::WindowCommand::SetMaximized;
use bevy::window::{close_on_esc, WindowMode};
use bevy::window::WindowMode::{BorderlessFullscreen, Fullscreen};

pub mod board;
use crate::board::board_core;
use crate::WindowPosition::Centered;

const FONT_ASSET_PATH: &str = ("OpenSans-Regular.ttf");


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "tetris.rs".to_string(),
            width: 1920.,
            height: 1080.,
            mode: WindowMode::SizedFullscreen, 
            
            
            ..default()})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(board_core::BoardPlugin)
        .add_system(close_on_esc)
        .run();
}

fn setup(mut commands : Commands){

    commands.spawn_bundle(Camera2dBundle::default());
}