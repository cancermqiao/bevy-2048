use std::cmp::Ordering;

use bevy::prelude::*;
use itertools::Itertools;

use crate::{borad::Board, game::Game};

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

pub fn tiles_shift(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points, &Children)>,
    mut texts: Query<&mut Text, With<TileText>>,
    query_board: Query<&Board>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
) {
    let shift_direction = keyboard_input
        .get_just_pressed()
        .find_map(|key_code| TileShift::try_from(key_code).ok());

    if let Some(tile_shift) = shift_direction {
        let mut it = tiles
            .iter_mut()
            .sorted_by(|a, b| tile_shift.sort(&a.1, &b.1))
            .peekable();
        let Ok(board) = query_board.get_single() else {
            return;
        };
        let mut column = 0;
        let mut tile_shift_or_merge_flag = false;
        while let Some(mut tile) = it.next() {
            tile_shift_or_merge_flag |=
                tile_shift.set_column_position(board.tile_num, &mut tile.1, column);
            if let Some(tile_next) = it.peek() {
                if tile_shift.get_row_position(&tile.1) != tile_shift.get_row_position(&tile_next.1)
                {
                    column = 0;
                } else if tile.2.value != tile_next.2.value {
                    column += 1;
                } else {
                    // merge tow tile
                    tile_shift_or_merge_flag = true;
                    let real_tile_next = it
                        .next()
                        .expect("A peeked tile should always exist when we .next here");
                    tile.2.value += real_tile_next.2.value;

                    // add score
                    game.score += tile.2.value;

                    if let Some(entity) = tile.3.first() {
                        let mut text = texts.get_mut(*entity).expect("expected Text to exist");
                        let text_section = text
                            .sections
                            .first_mut()
                            .expect("expected first section to be accessible as mutable");
                        let delta_length = tile.2.value.to_string().len() - text_section.value.len();
                        text_section.value = tile.2.value.to_string();
                        text_section.style.font_size -= delta_length as f32 * 5.0;
                    }
                    commands.entity(real_tile_next.0).despawn_recursive();

                    if let Some(future) = it.peek() {
                        if tile_shift.get_row_position(&tile.1)
                            != tile_shift.get_row_position(&future.1)
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
