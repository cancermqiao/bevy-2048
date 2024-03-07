use std::cmp::Ordering;

use bevy::{input::touch::TouchPhase, prelude::*};
use itertools::Itertools;

use crate::{board::Board, game::Game};

use super::{NewTileEvent, Points, Position, TileText};

enum TileShift {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&KeyCode> for TileShift {
    type Error = &'static str;

    fn try_from(key_code: &KeyCode) -> Result<Self, Self::Error> {
        match key_code {
            KeyCode::ArrowLeft => Ok(Self::Left),
            KeyCode::ArrowRight => Ok(Self::Right),
            KeyCode::ArrowUp => Ok(Self::Up),
            KeyCode::ArrowDown => Ok(Self::Down),
            _ => Err("Invalid tile_shift key code"),
        }
    }
}

impl TryFrom<Vec2> for TileShift {
    type Error = &'static str;

    fn try_from(delta_xy: Vec2) -> Result<Self, Self::Error> {
        if delta_xy.x.abs() > delta_xy.y.abs() {
            if delta_xy.x > 0.0 {
                Ok(Self::Right)
            } else {
                Ok(Self::Left)
            }
        } else if delta_xy.y > 0. {
            Ok(Self::Down)
        } else {
            Ok(Self::Up)
        }
    }
}

impl TileShift {
    fn sort(&self, a: &Position, b: &Position) -> Ordering {
        match self {
            Self::Left => match Ord::cmp(&a.y, &b.y) {
                Ordering::Equal => Ord::cmp(&a.x, &b.x),
                ordering => ordering,
            },
            Self::Right => match Ord::cmp(&b.y, &a.y) {
                Ordering::Equal => Ord::cmp(&b.x, &a.x),
                ordering => ordering,
            },
            Self::Up => match Ord::cmp(&a.x, &b.x) {
                Ordering::Equal => Ord::cmp(&b.y, &a.y),
                ordering => ordering,
            },
            Self::Down => match Ord::cmp(&b.x, &a.x) {
                Ordering::Equal => Ord::cmp(&a.y, &b.y),
                ordering => ordering,
            },
        }
    }

    fn set_column_position(&self, tile_num: u8, position: &mut Mut<Position>, index: u8) -> bool {
        match self {
            Self::Left => {
                if position.x != index {
                    position.x = index;
                    true
                } else {
                    false
                }
            }
            Self::Right => {
                if position.x != tile_num - index - 1 {
                    position.x = tile_num - index - 1;
                    true
                } else {
                    false
                }
            }
            Self::Up => {
                if position.y != tile_num - index - 1 {
                    position.y = tile_num - index - 1;
                    true
                } else {
                    false
                }
            }
            Self::Down => {
                if position.y != index {
                    position.y = index;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn get_row_position(&self, position: &Position) -> u8 {
        match self {
            Self::Left | Self::Right => position.y,
            Self::Up | Self::Down => position.x,
        }
    }
}

#[derive(Clone)]
pub struct TouchPosition {
    id: u64,
    start_position: Vec2,
    end_position: Vec2,
}

pub fn touch_input_system(
    mut commands: Commands,
    mut tiles: Query<(Entity, &mut Sprite, &mut Position, &mut Points, &Children)>,
    mut texts: Query<&mut Text, With<TileText>>,
    query_board: Query<&Board>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
    mut touch_event_reader: EventReader<TouchInput>,
    mut touch_positions: Local<Vec<TouchPosition>>,
) {
    for touch_event in touch_event_reader.read() {
        match touch_event.phase {
            TouchPhase::Started => touch_positions.push(TouchPosition {
                id: touch_event.id,
                start_position: touch_event.position,
                end_position: touch_event.position,
            }),
            TouchPhase::Moved => {
                // 触控移动，更新触控点的结束位置
                if let Some(position) = touch_positions.iter_mut().find(|p| p.id == touch_event.id)
                {
                    position.end_position = touch_event.position;
                }
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                // 触控结束，从本地状态中删除触控点，并计算滑动方向
                if let Some(position) = touch_positions
                    .iter()
                    .find(|p| p.id == touch_event.id)
                    .cloned()
                {
                    let delta_xy = position.end_position - position.start_position;
                    let swipe_direction = TileShift::try_from(delta_xy).ok();
                    tiles_shift(
                        swipe_direction,
                        &mut commands,
                        &mut tiles,
                        &mut texts,
                        &query_board,
                        &mut tile_writer,
                        &mut game,
                    )
                }
                touch_positions.retain(|p| p.id != touch_event.id);
            }
        }
    }
}

pub fn keyboard_input_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tiles: Query<(Entity, &mut Sprite, &mut Position, &mut Points, &Children)>,
    mut texts: Query<&mut Text, With<TileText>>,
    query_board: Query<&Board>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
) {
    let shift_direction = keyboard_input
        .get_just_pressed()
        .find_map(|key_code| TileShift::try_from(key_code).ok());
    tiles_shift(
        shift_direction,
        &mut commands,
        &mut tiles,
        &mut texts,
        &query_board,
        &mut tile_writer,
        &mut game,
    )
}

fn tiles_shift(
    shift_direction: Option<TileShift>,
    commands: &mut Commands,
    tiles: &mut Query<(Entity, &mut Sprite, &mut Position, &mut Points, &Children)>,
    texts: &mut Query<&mut Text, With<TileText>>,
    query_board: &Query<&Board>,
    tile_writer: &mut EventWriter<NewTileEvent>,
    game: &mut ResMut<Game>,
) {
    if let Some(tile_shift) = shift_direction {
        let mut it = tiles
            .iter_mut()
            .sorted_by(|a, b| tile_shift.sort(&a.2, &b.2))
            .peekable();
        let Ok(board) = query_board.get_single() else {
            return;
        };
        let mut column = 0;
        let mut tile_shift_or_merge_flag = false;
        while let Some(mut tile) = it.next() {
            tile_shift_or_merge_flag |=
                tile_shift.set_column_position(board.tile_num, &mut tile.2, column);
            if let Some(tile_next) = it.peek() {
                if tile_shift.get_row_position(&tile.2) != tile_shift.get_row_position(&tile_next.2)
                {
                    column = 0;
                } else if tile.3.value != tile_next.3.value {
                    column += 1;
                } else {
                    // merge two tile
                    tile_shift_or_merge_flag = true;
                    let real_tile_next = it
                        .next()
                        .expect("A peeked tile should always exist when we .next here");
                    tile.3.value += real_tile_next.3.value;
                    tile.1.color = tile.3.tile_color();

                    // add score
                    game.score += tile.3.value;

                    if let Some(entity) = tile.4.first() {
                        let mut text = texts.get_mut(*entity).expect("expected Text to exist");
                        let text_section = text
                            .sections
                            .first_mut()
                            .expect("expected first section to be accessible as mutable");
                        let delta_length =
                            tile.3.value.to_string().len() - text_section.value.len();
                        text_section.value = tile.3.value.to_string();
                        text_section.style.color = tile.3.text_color();
                        text_section.style.font_size -= delta_length as f32 * 5.0;
                    }
                    commands.entity(real_tile_next.0).despawn_recursive();

                    if let Some(future) = it.peek() {
                        if tile_shift.get_row_position(&tile.2)
                            != tile_shift.get_row_position(&future.2)
                        {
                            column = 0;
                        } else {
                            column += 1;
                        }
                    }
                }
            }
        }
        if tile_shift_or_merge_flag {
            tile_writer.send(NewTileEvent);
        }
    }
}
