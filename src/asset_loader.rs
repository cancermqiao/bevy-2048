use std::path::{Path, PathBuf};

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct FontSpec {
    pub family: Handle<Font>,
}

#[derive(Resource, Default)]
pub struct ButtonAssets {
    pub exit: ButtonState,
    pub play: ButtonState,
    pub pause: ButtonState,
    pub repeat: ButtonState,
}

impl ButtonAssets {
    fn new(asset_server: Res<AssetServer>, path: &Path) -> Self {
        Self {
            exit: ButtonState::new(&asset_server, path.join("exit")),
            play: ButtonState::new(&asset_server, path.join("play")),
            pause: ButtonState::new(&asset_server, path.join("pause")),
            repeat: ButtonState::new(&asset_server, path.join("repeat")),
        }
    }
}

#[derive(Default)]
pub struct ButtonState {
    pub idle: Handle<Image>,
    pub hover: Handle<Image>,
}

impl ButtonState {
    fn new(asset_server: &Res<AssetServer>, path: PathBuf) -> Self {
        Self {
            idle: asset_server.load(path.join("idle.png")),
            hover: asset_server.load(path.join("hover.png")),
        }
    }
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontSpec>()
            .init_resource::<ButtonAssets>()
            .add_systems(PreStartup, (load_font, load_button_assets));
    }
}

fn load_font(mut font_spec: ResMut<FontSpec>, asset_server: Res<AssetServer>) {
    *font_spec = FontSpec {
        family: asset_server.load("fonts/FiraSans-Bold.ttf"),
    }
}

fn load_button_assets(mut button_assets: ResMut<ButtonAssets>, asset_server: Res<AssetServer>) {
    let button_path = Path::new("button");
    *button_assets = ButtonAssets::new(asset_server, button_path);
}
