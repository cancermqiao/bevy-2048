use bevy::prelude::*;
use itertools::Itertools;

use crate::color::{board::BOARD, tile::TILE_PLACEHOLDER};

const TILE_NUM: u8 = 4;
pub const TILE_SIZE: f32 = 40.0;
const TILE_SPACE: f32 = 10.0;

#[derive(Component)]
pub struct Board {
    pub tile_num: u8,
    size: f32,
}

impl Board {
    fn new() -> Self {
        Self {
            tile_num: TILE_NUM,
            size: f32::from(TILE_NUM) * TILE_SIZE + f32::from(TILE_NUM + 1) * TILE_SPACE,
        }
    }

    pub fn tile_position_to_physical(&self, pos: u8) -> f32 {
        let offset = -self.size / 2.0 + 0.5 * TILE_SIZE;
        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACE
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board);
    }
}

fn spawn_board(mut commands: Commands) {
    let board = Board::new();
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: BOARD,
                custom_size: Some(Vec2::new(board.size, board.size)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for tile in (0..board.tile_num).cartesian_product(0..board.tile_num) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.tile_position_to_physical(tile.0),
                        board.tile_position_to_physical(tile.1),
                        0.0,
                    ),
                    ..default()
                });
            }
        })
        .insert(board);
}
