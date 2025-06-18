use bevy::prelude::*;

use super::AssetLoadSchedule;

#[derive(Resource, Default, Debug, Clone)]
pub struct IconAsset {
    pub bevy_icon: Handle<Image>,
}

pub struct IconPlugin;

impl Plugin for IconPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IconAsset>()
            .add_systems(AssetLoadSchedule, load_icons);
    }
}

fn load_icons(mut icon_asset: ResMut<IconAsset>, asset_server: Res<AssetServer>) {
    *icon_asset = IconAsset {
        bevy_icon: asset_server.load("branding/bevy_logo_light.png"),
    }
}
