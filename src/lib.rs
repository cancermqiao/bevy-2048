mod game;
mod input;
mod render;

use bevy::prelude::*;

use game::GamePlugin;
use input::InputPlugin;
use render::{RenderPlugin, WINDOW_H, WINDOW_W};

pub struct Game2048Plugin;

impl Plugin for Game2048Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(bevy::log::LogPlugin {
                    // Bevy/winit can emit a harmless window-destroy ordering warning on shutdown.
                    // Keep normal Bevy logs, but hide that noisy target unless it becomes an error.
                    filter: format!("{},bevy_winit::state=error", bevy::log::DEFAULT_FILTER),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "2048 - Bevy".to_string(),
                        resolution: (WINDOW_W as u32, WINDOW_H as u32).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .configure_sets(Update, (GameSet::Input, GameSet::Render).chain())
        .add_plugins((GamePlugin, InputPlugin, RenderPlugin));
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GameSet {
    Input,
    Render,
}
