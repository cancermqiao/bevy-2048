pub mod game;
pub mod paused_menu;
pub mod splash;

use bevy::prelude::*;
use game::GamePlugin;
use splash::SplashPlugin;

use crate::ui::paused_menu::PausedMenuPlugin;

pub const TEXT_COLOR: Color = Color::srgb_u8(119, 110, 101);

#[derive(Component)]
pub struct NewGameButton;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SplashPlugin, GamePlugin, PausedMenuPlugin));
    }
}
