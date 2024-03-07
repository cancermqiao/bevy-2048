mod asset_loader;
mod board;
mod camera;
mod color;
mod game;
mod state;
mod tiles;
mod ui;

use asset_loader::AssetLoaderPlugin;
use bevy::{prelude::*, window::WindowResolution};
use bevy_easings::EasingsPlugin;
use board::BoardPlugin;
use camera::CameraPlugin;
use color::BACKGROUPD_COLOR;
use game::GamePlugin;
use state::StatePlugin;
use tiles::TilesPlugin;
use ui::GameUiPlugin;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ClearColor(BACKGROUPD_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                resolution: WindowResolution::new(360.0, 640.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EasingsPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(BoardPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TilesPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(GameUiPlugin)
        .add_plugins(StatePlugin)
        .run()
}
