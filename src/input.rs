use bevy::{input::touch::*, prelude::*};

use crate::{
    asset_loader::fonts::FontAsset,
    schedule::GameRunningSet,
    score::ScoreRes,
    tiles::{Tile, generate_tiles},
    ui::game::GameBoard,
};

// 最小滑动距离阈值
const SWIPE_THRESHOLD: f32 = 50.0;

#[derive(Debug, Event)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&KeyCode> for MoveDirection {
    type Error = &'static str;

    fn try_from(key_code: &KeyCode) -> Result<Self, Self::Error> {
        match key_code {
            KeyCode::ArrowLeft => Ok(Self::Left),
            KeyCode::ArrowRight => Ok(Self::Right),
            KeyCode::ArrowUp => Ok(Self::Up),
            KeyCode::ArrowDown => Ok(Self::Down),
            _ => Err("Invalid MoveDirection KeyCode"),
        }
    }
}

impl TryFrom<&TouchPosition> for MoveDirection {
    type Error = &'static str;

    fn try_from(touch_position: &TouchPosition) -> Result<Self, Self::Error> {
        let delta = touch_position.end_position - touch_position.start_position;
        if delta.length_squared() < SWIPE_THRESHOLD.powi(2) {
            return Err("Swipe distance is too short");
        }
        if delta.x.abs() > delta.y.abs() {
            if delta.x > 0. {
                Ok(Self::Right)
            } else {
                Ok(Self::Left)
            }
        } else if delta.y > 0. {
            Ok(Self::Up)
        } else {
            Ok(Self::Down)
        }
    }
}

impl MoveDirection {
    fn horizontal(&self) -> bool {
        match self {
            Self::Left | Self::Right => true,
            Self::Up | Self::Down => false,
        }
    }

    fn forward(&self) -> bool {
        match self {
            Self::Left | Self::Up => true,
            Self::Right | Self::Down => false,
        }
    }

    pub fn move_tiles(
        &self,
        commands: &mut Commands,
        query: &mut Query<&mut Tile>,
        game_board: &mut GameBoard,
        score_res: &mut ScoreRes,
        font_asset: &FontAsset,
    ) {
        let mut tile_moved = false;
        let cols = game_board.cols;
        let rows: usize = game_board.rows;
        let (move_rows, move_cols) = if self.horizontal() {
            (rows, cols)
        } else {
            (cols, rows)
        };
        for row in 0..move_rows {
            let start = if self.horizontal() { row * cols } else { row };
            let end = if self.horizontal() {
                start + cols - 1
            } else {
                start + (rows - 1) * cols
            };
            let step = if self.horizontal() { 1 } else { cols };
            let mut i = if self.forward() { start } else { end };
            let mut pre = false;
            for k in 0..move_cols {
                let j = if self.forward() {
                    start + k * step
                } else {
                    end - k * step
                };
                if let Some(entity) = game_board.entities[j] {
                    if !pre {
                        if i != j {
                            game_board.entities.swap(i, j);
                            tile_moved = true;
                        }
                        pre = true;
                    } else if query.get(game_board.entities[i].unwrap()).unwrap().value
                        == query.get(entity).unwrap().value
                    {
                        query.get_mut(entity).unwrap().value *= 2;
                        score_res.score += query.get_mut(entity).unwrap().value;
                        commands.entity(game_board.entities[i].unwrap()).despawn();
                        game_board.entities[i] = Some(entity);
                        game_board.entities[j] = None;
                        pre = false;
                        if self.forward() {
                            i += step;
                        } else {
                            i -= step;
                        }
                        tile_moved = true;
                    } else {
                        if self.forward() {
                            i += step;
                        } else {
                            i -= step;
                        }
                        game_board.entities.swap(i, j);
                    }
                }
            }
        }
        if tile_moved {
            generate_tiles(commands, game_board, font_asset);
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveDirection>().add_systems(
            Update,
            (keyboard_input, touch_input).in_set(GameRunningSet::UserInput),
        );
    }
}

fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_direction_events: EventWriter<MoveDirection>,
) {
    if let Some(move_direction) = keyboard_input
        .get_just_pressed()
        .find_map(|key_code| MoveDirection::try_from(key_code).ok())
    {
        move_direction_events.write(move_direction);
    }
}

struct TouchPosition {
    id: u64,
    start_position: Vec2,
    end_position: Vec2,
}

fn touch_input(
    mut touch_event_reader: EventReader<TouchInput>,
    mut touch_positions: Local<Vec<TouchPosition>>,
    mut move_direction_events: EventWriter<MoveDirection>,
) {
    for ev in touch_event_reader.read() {
        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                touch_positions.push(TouchPosition {
                    id: ev.id,
                    start_position: ev.position,
                    end_position: ev.position,
                });
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                if let Some(position) = touch_positions.iter_mut().find(|p| p.id == ev.id) {
                    position.end_position = ev.position;
                    if let Ok(move_direction) = MoveDirection::try_from(&*position) {
                        move_direction_events.write(move_direction);
                    }
                }
                touch_positions.retain(|p| p.id != ev.id);
            }
            TouchPhase::Moved => {}
        }
    }
}
