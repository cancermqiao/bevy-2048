mod asset_loader;
mod button;
mod camera;
mod fps;
mod input;
mod schedule;
mod score;
mod state;
mod tiles;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(fps::FpsPlugin)
        .add_plugins(score::ScorePlugin)
        .add_plugins(tiles::TilesPlugin)
        .add_plugins(button::ButtonPlugins)
        .add_plugins(input::InputPlugin)
        .add_plugins(state::StatePlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(schedule::SchedulePlugin)
        .run();
}
