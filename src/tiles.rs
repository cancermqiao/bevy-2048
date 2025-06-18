use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    asset_loader::fonts::FontAsset,
    input::MoveDirection,
    schedule::GameRunningSet,
    score::ScoreRes,
    state::{AppState, InGame},
    ui::game::{self, BACKGROUND_TILE_SIZE, GameBoard},
};

pub const TILE_SIZE: f32 = 110.0;
pub const TILE_RADIUS: f32 = 10.0;
const TILE_FONT_SIZE: f32 = 36.6;

const SPEED: f32 = 2000.0;

#[derive(Component)]
pub struct Tile {
    pub value: usize,
}

impl Tile {
    pub fn color(&self) -> Color {
        match self.value {
            2 => Color::srgb_u8(238, 228, 218),
            4 => Color::srgb_u8(237, 224, 200),
            8 => Color::srgb_u8(242, 177, 121),
            16 => Color::srgb_u8(245, 149, 99),
            32 => Color::srgb_u8(246, 124, 95),
            64 => Color::srgb_u8(246, 94, 59),
            128 => Color::srgb_u8(237, 207, 114),
            256 => Color::srgb_u8(237, 204, 97),
            512 => Color::srgb_u8(237, 200, 80),
            1024 => Color::srgb_u8(237, 197, 63),
            2048 => Color::srgb_u8(237, 194, 46),
            4096 => Color::srgb_u8(255, 102, 0),
            8192 => Color::srgb_u8(255, 51, 0),
            16384 => Color::srgb_u8(255, 0, 0),
            32768 => Color::srgb_u8(153, 0, 0),
            65536 => Color::srgb_u8(102, 0, 0),
            _ => Color::srgb_u8(51, 0, 0),
        }
    }

    pub fn text_color(&self) -> Color {
        if self.value > 4 {
            // light
            Color::srgb_u8(250, 248, 239)
        } else {
            // dark
            Color::srgb_u8(100, 90, 80)
        }
    }
}

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_tiles, init_tiles)
                .chain()
                .run_if(in_state(InGame::Init)),
        )
        .add_systems(
            Update,
            (move_tiles, game_over)
                .chain()
                .in_set(GameRunningSet::EntityUpdate),
        )
        .add_systems(Update, render_tiles.run_if(in_state(AppState::InGame)));
    }
}

fn init_tiles(
    mut commands: Commands,
    mut game_board: ResMut<game::GameBoard>,
    font_asset: Res<FontAsset>,
    mut next_state: ResMut<NextState<InGame>>,
) {
    for _ in 0..2 {
        generate_tiles(&mut commands, &mut game_board, &font_asset);
    }
    next_state.set(InGame::Running);
}

pub fn generate_tiles(
    commands: &mut Commands,
    game_board: &mut game::GameBoard,
    font_asset: &FontAsset,
) {
    let mut rng = rand::rng();
    let random_index = game_board
        .entities
        .iter()
        .enumerate()
        .filter(|(_, x)| x.is_none())
        .map(|(i, _)| i)
        .choose(&mut rng)
        .unwrap();
    let pos = game_board.translations[random_index];
    // 90% chance of 2, 10% chance of 4
    let tile = if rng.random_ratio(1, 10) {
        Tile { value: 4 }
    } else {
        Tile { value: 2 }
    };

    let entity = commands
        .spawn((
            Node {
                width: Val::Px(TILE_SIZE),
                height: Val::Px(TILE_SIZE),
                position_type: PositionType::Absolute,
                left: Val::Px((pos.x - BACKGROUND_TILE_SIZE) / 2.0),
                top: Val::Px((pos.y - BACKGROUND_TILE_SIZE) / 2.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(TILE_RADIUS)),
            BackgroundColor(tile.color()),
            children![(
                Text::new(tile.value.to_string()),
                TextFont {
                    font: font_asset.extra_bold.clone(),
                    font_size: TILE_FONT_SIZE,
                    ..default()
                },
                TextColor(tile.text_color()),
            )],
            GlobalZIndex(1),
        ))
        .insert(tile)
        .id();

    game_board.entities[random_index] = Some(entity);
}

fn move_tiles(
    mut commands: Commands,
    mut query: Query<&mut Tile>,
    mut move_direction_reader: EventReader<MoveDirection>,
    mut game_board: ResMut<GameBoard>,
    mut score_res: ResMut<ScoreRes>,
    font_asset: Res<FontAsset>,
) {
    for move_direction in move_direction_reader.read() {
        move_direction.move_tiles(
            &mut commands,
            &mut query,
            &mut game_board,
            &mut score_res,
            &font_asset,
        );
    }
}

fn render_tiles(
    mut query: Query<(&Tile, &mut Node, &mut BackgroundColor, &Children)>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
    game_board: Res<game::GameBoard>,
    timer: Res<Time>,
) {
    for (i, entity) in game_board
        .entities
        .iter()
        .enumerate()
        .filter(|(_, entity)| entity.is_some())
    {
        if let Ok((tile, mut node, mut background_color, children)) = query.get_mut(entity.unwrap())
        {
            if let (Val::Px(left_val), Val::Px(top_val)) = (node.left, node.top) {
                let pos = game_board.translations[i];
                let target_left = (pos.x - BACKGROUND_TILE_SIZE) / 2.0;
                let target_top = (pos.y - BACKGROUND_TILE_SIZE) / 2.0;
                let delta_movement = timer.delta_secs() * SPEED;

                // Update node.left
                if left_val < target_left {
                    let distance = target_left - left_val;
                    node.left = Val::Px(left_val + delta_movement.min(distance));
                } else if left_val > target_left {
                    let distance = left_val - target_left;
                    node.left = Val::Px(left_val - delta_movement.min(distance));
                }
                // Update node.top
                if top_val < target_top {
                    let distance = target_top - top_val;
                    node.top = Val::Px(top_val + delta_movement.min(distance));
                } else if top_val > target_top {
                    let distance = top_val - target_top;
                    node.top = Val::Px(top_val - delta_movement.min(distance));
                }
            }
            background_color.0 = tile.color();

            if let Ok((mut text, mut text_color)) = text_query.get_mut(*children.first().unwrap()) {
                text.0 = tile.value.to_string();
                text_color.0 = tile.text_color();
            }
        }
    }
}

fn game_over(
    mut game_state: ResMut<NextState<InGame>>,
    tile_query: Query<&Tile>,
    game_board: Res<GameBoard>,
) {
    if game_board.is_changed() && game_board.entities.iter().all(|x| x.is_some()) {
        for r in 0..game_board.rows {
            for c in 0..game_board.cols {
                if r < game_board.rows - 1 {
                    let entity = game_board.entities[r * game_board.cols + c].unwrap();
                    let down_entity = game_board.entities[(r + 1) * game_board.cols + c].unwrap();
                    if tile_query.get(entity).unwrap().value
                        == tile_query.get(down_entity).unwrap().value
                    {
                        return;
                    }
                }

                if c < game_board.cols - 1 {
                    let entity = game_board.entities[r * game_board.cols + c].unwrap();
                    let right_entity = game_board.entities[r * game_board.cols + c + 1].unwrap();
                    if tile_query.get(entity).unwrap().value
                        == tile_query.get(right_entity).unwrap().value
                    {
                        return;
                    }
                }
            }
        }
        game_state.set(InGame::GameOver);
    }
}

fn despawn_tiles(mut commands: Commands, mut game_board: ResMut<game::GameBoard>) {
    for entity in game_board.entities.iter().flatten() {
        commands.entity(*entity).despawn();
    }
    game_board.reset();
}
