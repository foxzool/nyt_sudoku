#![allow(clippy::type_complexity)]

pub mod color;
mod game;
mod loading;
mod menu;
mod share;

use crate::game::SudokuPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use crate::color::WHITE_COLOR;
use bevy::app::App;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate game
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(ClearColor(WHITE_COLOR))
            .add_plugins((LoadingPlugin, MenuPlugin, SudokuPlugin));

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        // }
    }
}
