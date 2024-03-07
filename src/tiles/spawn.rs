use bevy::prelude::*;
use itertools::Itertools;
use rand::seq::IteratorRandom;

use crate::{
    asset_loader::FontSpec,
    borad::{Board, TILE_SIZE},
    color::tile::TILE,
};

use super::{Points, Position, TileText};

pub fn init_tiles(mut commands: Commands, query_board: Query<&Board>, font_spec: Res<FontSpec>) {
    let Ok(board) = query_board.get_single() else {
        return;
    };

    let mut rng = rand::thread_rng();
    let starting_tiles = (0..board.tile_num)
        .cartesian_product(0..board.tile_num)
        .choose_multiple(&mut rng, 2);

    for &(x, y) in starting_tiles.iter() {
        let pos: Position = Position::new(x, y);
        spawn_tile(&mut commands, board, pos, &font_spec);
    }
}

pub fn spawn_tile(
    commands: &mut Commands,
    board: &Board,
    pos: Position,
    font_spec: &Res<FontSpec>,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: TILE,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                board.tile_position_to_physical(pos.x),
                board.tile_position_to_physical(pos.y),
                1.0,
            ),
            ..default()
        })
        .with_children(|child_builder| {
            child_builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        "2",
                        TextStyle {
                            font: font_spec.family.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                })
                .insert(TileText);
        })
        .insert(Points::new(2))
        .insert(pos);
}
