use bevy::prelude::*;

use crate::{
    asset_loader::{fonts::FontAsset, icons::IconAsset},
    state::AppState,
};

use super::TEXT_COLOR;

#[derive(Component)]
struct OnSplashScreen;

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), setup_splash);
    }
}

fn setup_splash(mut commands: Commands, icon_asset: Res<IconAsset>, font_asset: Res<FontAsset>) {
    commands.spawn((
        StateScoped(AppState::Splash),
        Node {
            margin: UiRect::all(Val::Auto),
            align_self: AlignSelf::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        OnSplashScreen,
        children![
            (
                Text::new("2048"),
                TextColor(TEXT_COLOR),
                TextFont {
                    font: font_asset.extra_bold.clone(),
                    font_size: 100.0,
                    ..default()
                },
                Node {
                    margin: UiRect::all(Val::Auto),
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ),
            (
                ImageNode::new(icon_asset.bevy_icon.clone()),
                Node {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(100.0),
                    align_self: AlignSelf::Center,
                    ..default()
                },
            )
        ],
    ));
}
