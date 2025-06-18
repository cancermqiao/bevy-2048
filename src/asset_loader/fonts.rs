use bevy::prelude::*;

use crate::asset_loader::AssetLoadSchedule;

#[derive(Default, Resource, Clone, Debug)]
pub struct FontAsset {
    pub regular: Handle<Font>,
    pub medium: Handle<Font>,
    pub semi_bold: Handle<Font>,
    pub bold: Handle<Font>,
    pub extra_bold: Handle<Font>,
}

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAsset>()
        .add_systems(AssetLoadSchedule, load_fonts);

    }
}

fn load_fonts(mut font_asset: ResMut<FontAsset>, asset_server: Res<AssetServer>) {
    *font_asset = FontAsset {
        regular: asset_server.load("font/Montserrat-Regular.ttf"),
        medium: asset_server.load("font/Montserrat-Medium.ttf"),
        semi_bold: asset_server.load("font/Montserrat-SemiBold.ttf"),
        bold: asset_server.load("font/Montserrat-Bold.ttf"),
        extra_bold: asset_server.load("font/Montserrat-ExtraBold.ttf"),
    }
}