use bevy::prelude::*;

use crate::GameSet;
use crate::game::{Direction, Game};

const MIN_SWIPE_DISTANCE: f32 = 48.0;
const MOVE_INPUT_COOLDOWN_SECONDS: f32 = 0.18;

pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input.in_set(GameSet::Input));
    }
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    time: Res<Time>,
    mut move_lock: Local<f32>,
    mut game: ResMut<Game>,
) {
    *move_lock = (*move_lock - time.delta_secs()).max(0.0);

    // Keyboard input is handled first so desktop controls stay responsive and predictable.
    if keys.just_pressed(KeyCode::KeyR) {
        game.reset();
        *move_lock = 0.0;
        return;
    }

    if *move_lock > 0.0 {
        return;
    }

    let direction = if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        Some(Direction::Left)
    } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        Some(Direction::Right)
    } else if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        Some(Direction::Up)
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        Some(Direction::Down)
    } else {
        None
    };

    if let Some(direction) = direction {
        if game.make_move(direction) {
            *move_lock = MOVE_INPUT_COOLDOWN_SECONDS;
        }
        return;
    }

    // Touch input uses the release distance to convert a swipe into one board move.
    for touch in touches.iter_just_released() {
        let swipe = touch.distance();
        if swipe.length() < MIN_SWIPE_DISTANCE {
            continue;
        }

        let direction = if swipe.x.abs() > swipe.y.abs() {
            if swipe.x > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else if swipe.y > 0.0 {
            Direction::Up
        } else {
            Direction::Down
        };

        if game.make_move(direction) {
            *move_lock = MOVE_INPUT_COOLDOWN_SECONDS;
        }
        break;
    }
}
