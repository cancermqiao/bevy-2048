mod asset_loader;
mod borad;
mod camera;
mod color;
mod game;
mod state;
mod tiles;
mod ui;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy_easings::EasingsPlugin;
use borad::BoardPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use state::StatePlugin;
use tiles::TilesPlugin;
use ui::GameUiPlugin;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ClearColor(Color::hex("#1f2638").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EasingsPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(BoardPlugin)
        .add_plugins(TilesPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(GameUiPlugin)
        .add_plugins(StatePlugin)
        .run()
}
