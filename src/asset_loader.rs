use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct FontSpec {
    pub family: Handle<Font>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontSpec>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut font_spec: ResMut<FontSpec>, asset_server: Res<AssetServer>) {
    *font_spec = FontSpec {
        family: asset_server.load("fonts/FiraSans-Bold.ttf"),
    }

}
