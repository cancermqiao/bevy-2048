use bevy::prelude::*;

pub const SCORE_BOX: Color = Color::rgba(0.7, 0.68, 0.62, 1.0);

pub const SCORE_FONT_SIZE: f32 = 20.0;

pub const SCORE_BOARD_PADDING: UiRect = UiRect {
    left: Val::Px(20.0),
    right: Val::Px(20.0),
    top: Val::Px(15.0),
    bottom: Val::Px(15.0),
};