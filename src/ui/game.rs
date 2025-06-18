use std::cmp::Ordering;

use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    asset_loader::{fonts::FontAsset, icons::IconAsset}, score::ScoreRes, state::{AppState, InGame}, tiles::{TILE_RADIUS, TILE_SIZE}, ui::{NewGameButton, TEXT_COLOR}
};

pub const BACKGROUND_TILE_SIZE: f32 = TILE_SIZE - 2.0;
const SCORE_TITLE_SIZE: f32 = 12.0;
const SCORE_TEXT_SIZE: f32 = 23.0;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct BestText;

#[derive(Component)]
pub struct GameStateText;

#[derive(Component)]
pub struct BackGroundTile;

#[derive(Resource)]
pub struct GameBoard {
    pub rows: usize,
    pub cols: usize,
    pub translations: Vec<Vec3>,
    pub entities: Vec<Option<Entity>>,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            rows: 4,
            cols: 4,
            translations: vec![],
            entities: vec![None; 16],
        }
    }
}

impl GameBoard {
    pub fn reset(&mut self) {
        self.entities = vec![None; self.rows * self.cols];
    }
}

impl std::fmt::Debug for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameBoard")
            .field("rows", &self.rows)
            .field("cols", &self.cols)
            .field("translations", &self.translations)
            .field(
                "entities",
                &format_args!("\n{}", {
                    let mut board_display = String::new();
                    for r in 0..self.rows {
                        board_display.push_str("  [");
                        for c in 0..self.cols {
                            let index = r * self.cols + c;
                            match self.entities.get(index).and_then(|opt_e| *opt_e) {
                                Some(entity) => {
                                    board_display.push_str(&format!("{:?} ", entity.index()))
                                } // Display entity index
                                None => board_display.push_str("None "),
                            }
                        }
                        // board_display.trim_end_matches(' '); // Remove trailing space
                        board_display.push_str("]\n");
                    }
                    board_display
                }),
            )
            .finish()
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameBoard>()
            .add_systems(Update, setup_game.run_if(in_state(AppState::GameSetup)))
            .add_systems(OnExit(AppState::GameSetup), init_transform)
            .add_systems(Update, update_game_state.run_if(in_state(AppState::InGame)));
    }
}

fn init_transform(
    query: Query<&GlobalTransform, With<BackGroundTile>>,
    mut game_board: ResMut<GameBoard>,
) {
    for transform in query.iter().sort_by::<&GlobalTransform>(|a, b| {
        a.translation()
            .y
            .partial_cmp(&b.translation().y)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                a.translation()
                    .x
                    .partial_cmp(&b.translation().x)
                    .unwrap_or(Ordering::Equal)
            })
    }) {
        game_board.translations.push(transform.translation());
    }
}

fn setup_game(
    mut commands: Commands,
    icon_asset: Res<IconAsset>,
    game_board: ResMut<GameBoard>,
    font_asset: Res<FontAsset>,
    score_res: Res<ScoreRes>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    commands
        .spawn((Node {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            padding: UiRect::all(Val::Percent(2.0)),
            row_gap: Val::Percent(2.0),
            ..default()
        },))
        .with_children(|builder| {
            // Header
            builder
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![
                        GridTrack::fr(1.0),
                        GridTrack::min_content(),
                        GridTrack::fr(1.0),
                    ],
                    column_gap: Val::Px(4.0),
                    padding: UiRect {
                        left: Val::Px(8.0),
                        right: Val::Px(8.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    // Logo
                    spawn_nested_text_bundle(builder, "2048", &font_asset);
                    // Score Board
                    spawn_score_board(builder, &font_asset, &score_res);
                    // New Game Button
                    spawn_nested_button_bundle(builder, "New Game", &font_asset);
                });

            // Main Game Area
            builder
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(6.0)),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|builder| {
                    // game board
                    spawn_game_board(builder, game_board);
                });

            // GameState
            builder.spawn((
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                Text::new(""),
                TextFont {
                    font: font_asset.semi_bold.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                GameStateText,
            ));

            // Footer
            builder.spawn((
                ImageNode::new(icon_asset.bevy_icon.clone()),
                Node {
                    width: Val::Percent(5.0),
                    align_self: AlignSelf::FlexStart,
                    ..default()
                },
            ));
        });

    next_state.set(AppState::InGame);
}

fn spawn_score_board(builder: &mut ChildSpawnerCommands, font_asset: &FontAsset, score_res: &ScoreRes) {
    builder
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Percent(8.0),
            ..default()
        })
        .with_children(|builder| {
            // current score
            builder.spawn((
                Node {
                    padding: UiRect {
                        left: Val::Px(30.0),
                        right: Val::Px(30.0),
                        top: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                    },
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderRadius::all(Val::Px(15.0)),
                BackgroundColor(Color::srgb_u8(234, 231, 217)),
                children![
                    (
                        Text::new("SCORE"),
                        TextFont {
                            font: font_asset.semi_bold.clone(),
                            font_size: SCORE_TITLE_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ),
                    (
                        Text::new(score_res.score.to_string()),
                        TextFont {
                            font: font_asset.extra_bold.clone(),
                            font_size: SCORE_TEXT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        ScoreText
                    ),
                ],
            ));
            // best score
            builder.spawn((
                Node {
                    padding: UiRect {
                        left: Val::Px(30.0),
                        right: Val::Px(30.0),
                        top: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                    },
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderRadius::all(Val::Px(15.0)),
                BorderColor(Color::srgb_u8(234, 231, 217)),
                children![
                    (
                        Text::new("BEST"),
                        TextFont {
                            font: font_asset.semi_bold.clone(),
                            font_size: SCORE_TITLE_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ),
                    (
                        Text::new(score_res.best.to_string()),
                        TextFont {
                            font: font_asset.extra_bold.clone(),
                            font_size: SCORE_TEXT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        BestText
                    ),
                ],
            ));
        });
}

fn spawn_game_board(builder: &mut ChildSpawnerCommands, game_board: ResMut<GameBoard>) {
    builder
        .spawn((
            Node {
                display: Display::Grid,
                row_gap: Val::Px(12.0),
                column_gap: Val::Px(12.0),
                padding: UiRect::all(Val::Px(12.0)),
                grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                ..default()
            },
            BorderRadius::all(Val::Px(15.0)),
            BackgroundColor(Color::srgb_u8(104, 89, 74)),
        ))
        .with_children(|builder| {
            // spawn tiles
            for _ in 0..game_board.rows {
                for _ in 0..game_board.cols {
                    builder.spawn((
                        BackGroundTile,
                        Node {
                            width: Val::Px(BACKGROUND_TILE_SIZE),
                            height: Val::Px(BACKGROUND_TILE_SIZE),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(TILE_RADIUS)),
                        BackgroundColor(Color::srgb(0.7306, 0.6758, 0.6028)),
                    ));
                }
            }
        });
}

fn spawn_nested_text_bundle(
    builder: &mut ChildSpawnerCommands,
    text: &str,
    font_asset: &FontAsset,
) {
    builder.spawn((
        Text::new(text),
        TextColor(Color::srgb_u8(104, 89, 74)),
        TextFont {
            font: font_asset.bold.clone(),
            font_size: 50.0,
            ..default()
        },
    ));
}

fn spawn_nested_button_bundle(
    builder: &mut ChildSpawnerCommands,
    text: &str,
    font_asset: &FontAsset,
) {
    builder.spawn((
        Button,
        Node {
            padding: UiRect {
                left: Val::Px(20.0),
                right: Val::Px(20.0),
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
            },
            flex_direction: FlexDirection::Column,
            justify_self: JustifySelf::End,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        NewGameButton,
        BorderRadius::all(Val::Px(20.0)),
        BackgroundColor(Color::srgb_u8(148, 133, 118)), // 更浅的颜色
        children![(
            Text::new(text),
            TextFont {
                font: font_asset.medium.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(WHITE.into()),
        )],
    ));
}

fn update_game_state(mut query: Query<&mut Text, With<GameStateText>>, state: Res<State<InGame>>) {
    for mut text in query.iter_mut() {
        text.0 = state.to_string();
    }
}
