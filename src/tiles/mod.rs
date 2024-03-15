mod shift;
mod spawn;

use bevy::{prelude::*, utils::HashMap};
use bevy_easings::{Ease, EaseFunction, EasingType};
use itertools::Itertools;
use rand::seq::IteratorRandom;

use crate::{
    asset_loader::FontSpec,
    board::Board,
    color::tile::{
        TILE_1024, TILE_128, TILE_16, TILE_2, TILE_2048, TILE_256, TILE_32, TILE_4, TILE_512,
        TILE_64, TILE_8, TILE_SUPER, TILE_TEXT_COLOR_DARK, TILE_TEXT_COLOR_LIGHT,
    },
    state::GameState,
};

use self::{
    shift::{keyboard_input_system, touch_input_system},
    spawn::{init_tiles, spawn_tile},
};

#[derive(Component, Debug, PartialEq)]
pub struct Points {
    value: u32,
}

impl Points {
    fn new(value: u32) -> Self {
        Self { value }
    }

    fn tile_color(&self) -> Color {
        match self.value {
            2 => TILE_2,
            4 => TILE_4,
            8 => TILE_8,
            16 => TILE_16,
            32 => TILE_32,
            64 => TILE_64,
            128 => TILE_128,
            256 => TILE_256,
            512 => TILE_512,
            1024 => TILE_1024,
            2048 => TILE_2048,
            _ => TILE_SUPER,
        }
    }

    fn text_color(&self) -> Color {
        if self.value > 4 {
            TILE_TEXT_COLOR_LIGHT
        } else {
            TILE_TEXT_COLOR_DARK
        }
    }
}

#[derive(Component, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
struct TileText;

#[derive(Event)]
struct NewTileEvent;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init_tiles)
            .add_systems(OnExit(GameState::GameOver), init_tiles)
            .add_systems(
                Update,
                (
                    (keyboard_input_system, touch_input_system),
                    render_tiles,
                    new_tile_handler,
                    end_game,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_event::<NewTileEvent>();
    }
}

fn render_tiles(
    mut commands: Commands,
    tiles: Query<(Entity, &mut Transform, &Position), Changed<Position>>,
    query_board: Query<&Board>,
) {
    let Ok(board) = query_board.get_single() else {
        return;
    };
    for (entity, transform, pos) in tiles.iter() {
        commands.entity(entity).insert(transform.ease_to(
            Transform::from_xyz(
                board.tile_position_to_physical_x(pos.x),
                board.tile_position_to_physical_y(pos.y),
                transform.translation.z,
            ),
            EaseFunction::QuadraticInOut,
            EasingType::Once {
                duration: std::time::Duration::from_millis(100),
            },
        ));
    }
}

fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut commands: Commands,
    query_board: Query<&Board>,
    tiles: Query<&Position>,
    font_spec: Res<FontSpec>,
) {
    let Ok(board) = query_board.get_single() else {
        return;
    };
    for _event in tile_reader.read() {
        let mut rng = rand::thread_rng();
        let possible_positions = (0..board.tile_num)
            .cartesian_product(0..board.tile_num)
            .filter_map(|tile_pos| {
                let new_pos = Position::new(tile_pos.0, tile_pos.1);
                match tiles.iter().find(|pos| pos == &&new_pos) {
                    Some(_) => None,
                    None => Some(new_pos),
                }
            })
            .choose(&mut rng);

        if let Some(pos) = possible_positions {
            spawn_tile(&mut commands, board, pos, &font_spec)
        }
    }
}

fn end_game(
    tiles: Query<(&Position, &Points)>,
    query: Query<&Board>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if tiles.iter().len() == 16 {
        let board = query.single();
        let map: HashMap<&Position, &Points> = tiles.iter().collect();
        let neighbor_points = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let board_range = 0..(board.tile_num as i8);
        let has_move = tiles.iter().any(|(Position { x, y }, value)| {
            neighbor_points
                .iter()
                .filter_map(|(dx, dy)| {
                    let neighbor_x = *x as i8 + dx;
                    let neighbor_y = *y as i8 + dy;

                    if !board_range.contains(&neighbor_x) || !board_range.contains(&neighbor_y) {
                        return None;
                    };

                    map.get(&Position {
                        x: neighbor_x.try_into().unwrap(),
                        y: neighbor_y.try_into().unwrap(),
                    })
                })
                .any(|&v| v == value)
        });
        if !has_move {
            dbg!("game over!");
            next_state.set(GameState::GameOver)
        }
    }
}
