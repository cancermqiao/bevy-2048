use bevy::{
    color::palettes::css::RED,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::ui::TEXT_COLOR;

#[derive(Component)]
struct FpsText;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_fps)
            .add_systems(Update, fps_text_update);
    }
}

fn setup_fps(mut commands: Commands) {
    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(12.0),
                right: Val::Px(12.0),
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(RED.into()),
            FpsText,
        ));
}

fn fps_text_update(
    mut query: Query<&mut TextSpan, With<FpsText>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut span in query.iter_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of fps
                **span = format!("{value:.2}");
            }
        }
    }
}
