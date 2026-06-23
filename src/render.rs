use bevy::app::AppExit;
use bevy::prelude::*;

use crate::GameSet;
use crate::game::{GRID, Game};

pub const WINDOW_W: f32 = 720.0;
pub const WINDOW_H: f32 = 860.0;

const BOARD_SIZE: f32 = 560.0;
const TILE_GAP: f32 = 14.0;
const TILE_SIZE: f32 = (BOARD_SIZE - TILE_GAP * 5.0) / GRID as f32;
const TILE_ANIMATION_SECONDS: f32 = 0.18;
const TILE_POP_SECONDS: f32 = 0.08;

pub(crate) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(color_bg()))
            .init_resource::<UiMotion>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, handle_modal_buttons.in_set(GameSet::Input))
            .add_systems(Update, render_game.in_set(GameSet::Render));
    }
}

#[derive(Resource, Default)]
struct UiMotion {
    // Tracks Game::revision so visual feedback restarts only after a real state change.
    last_revision: u64,
    elapsed: f32,
}

#[derive(Resource)]
struct GameFonts {
    regular: Handle<Font>,
    medium: Handle<Font>,
    bold: Handle<Font>,
}

#[derive(Component)]
struct RenderedUi;

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct CloseButton;

type ButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        Option<&'static RestartButton>,
        Option<&'static CloseButton>,
    ),
    (Changed<Interaction>, With<Button>),
>;

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(GameFonts {
        regular: asset_server.load("fonts/ClearSans-Regular.ttf"),
        medium: asset_server.load("fonts/ClearSans-Medium.ttf"),
        bold: asset_server.load("fonts/ClearSans-Bold.ttf"),
    });
}

fn render_game(
    mut commands: Commands,
    rendered: Query<Entity, With<RenderedUi>>,
    game: Res<Game>,
    fonts: Res<GameFonts>,
    time: Res<Time>,
    mut motion: ResMut<UiMotion>,
) {
    for entity in &rendered {
        // Despawning the root also removes the full UI subtree through the Children relationship.
        commands.entity(entity).despawn();
    }

    if motion.last_revision != game.revision {
        motion.last_revision = game.revision;
        motion.elapsed = 0.0;
    } else {
        motion.elapsed += time.delta_secs();
    }

    let tile_progress = tile_motion_progress(motion.elapsed);
    let is_animating = tile_progress < 1.0 && !game.motions.is_empty();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(18.0),
                ..default()
            },
            BackgroundColor(color_bg()),
            RenderedUi,
        ))
        .with_children(|root| {
            spawn_header(root, &game, &fonts);
            spawn_board(
                root,
                &game,
                tile_progress,
                is_animating,
                motion.elapsed,
                &fonts,
            );

            if game.game_over {
                spawn_game_over_modal(root, &fonts);
            }
        });
}

fn handle_modal_buttons(
    mut interactions: ButtonInteractionQuery,
    mut game: ResMut<Game>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (interaction, mut color, restart, close) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                if restart.is_some() {
                    game.reset();
                }
                if close.is_some() {
                    app_exit.write(AppExit::Success);
                }
                *color = button_pressed_color().into();
            }
            Interaction::Hovered => {
                *color = button_hover_color().into();
            }
            Interaction::None => {
                *color = button_color().into();
            }
        }
    }
}

fn spawn_header(parent: &mut ChildSpawnerCommands, game: &Game, fonts: &GameFonts) {
    parent
        .spawn(Node {
            width: Val::Px(BOARD_SIZE),
            height: Val::Px(104.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|header| {
            spawn_title_badge(header, fonts);

            header
                .spawn(Node {
                    width: Val::Px(244.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|actions| {
                    actions
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|scores| {
                            spawn_score_card(scores, "SCORE", game.score, 118.0, fonts);
                            spawn_score_card(scores, "BEST", game.best_score, 110.0, fonts);
                        });
                    spawn_header_restart_button(actions, fonts);
                });
        });
}

fn spawn_title_badge(parent: &mut ChildSpawnerCommands, fonts: &GameFonts) {
    parent
        .spawn(Node {
            width: Val::Px(190.0),
            height: Val::Px(70.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|badge| {
            spawn_text(badge, "2048", 66.0, color_title(), fonts);
        });
}

fn spawn_header_restart_button(parent: &mut ChildSpawnerCommands, fonts: &GameFonts) {
    parent
        .spawn((
            Button,
            RestartButton,
            Node {
                width: Val::Px(158.0),
                height: Val::Px(48.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(button_color()),
        ))
        .with_children(|button| {
            spawn_text_with_weight(
                button,
                "New Game",
                19.0,
                color_light_text(),
                FontWeight::MEDIUM,
                fonts,
            );
        });
}

fn spawn_score_card(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    value: u32,
    width: f32,
    fonts: &GameFonts,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(width),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(2.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(score_card_color()),
            BorderColor::all(score_card_border_color()),
        ))
        .with_children(|card| {
            spawn_text_with_weight(
                card,
                label,
                13.0,
                color_score_label(),
                FontWeight::BOLD,
                fonts,
            );
            spawn_text_with_weight(
                card,
                value.to_string(),
                25.0,
                color_score_value(),
                FontWeight::BOLD,
                fonts,
            );
        });
}

fn spawn_board(
    parent: &mut ChildSpawnerCommands,
    game: &Game,
    tile_progress: f32,
    is_animating: bool,
    motion_elapsed: f32,
    fonts: &GameFonts,
) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Relative,
                width: Val::Px(BOARD_SIZE),
                height: Val::Px(BOARD_SIZE),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip(),
                row_gap: Val::Px(TILE_GAP),
                padding: UiRect::all(Val::Px(TILE_GAP)),
                border_radius: BorderRadius::all(Val::Px(26.0)),
                ..default()
            },
            BackgroundColor(color_board()),
        ))
        .with_children(|board| {
            for row in 0..GRID {
                board
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(TILE_SIZE),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(TILE_GAP),
                        ..default()
                    })
                    .with_children(|row_node| {
                        for col in 0..GRID {
                            let value = if is_animating && should_hide_static_tile(game, row, col) {
                                0
                            } else {
                                game.board[row][col]
                            };
                            let scale = tile_pop_scale(game, row, col, motion_elapsed);
                            spawn_tile(row_node, value, scale, fonts);
                        }
                    });
            }

            if is_animating {
                spawn_moving_tiles(board, game, tile_progress, fonts);
            }
        });
}

fn spawn_tile(parent: &mut ChildSpawnerCommands, value: u32, scale: f32, fonts: &GameFonts) {
    let tile_size = TILE_SIZE * scale;

    parent
        .spawn(Node {
            width: Val::Px(TILE_SIZE),
            height: Val::Px(TILE_SIZE),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|slot| {
            slot.spawn((
                Node {
                    width: Val::Px(tile_size),
                    height: Val::Px(tile_size),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(tile_color(value)),
                tile_shadow(value),
            ))
            .with_children(|tile| {
                if value != 0 {
                    let font_size = match value.to_string().len() {
                        1 => 62.0,
                        2 => 60.0,
                        3 => 54.0,
                        4 => 48.0,
                        _ => 42.0,
                    };
                    let color = if value <= 4 {
                        color_ink()
                    } else {
                        color_light_text()
                    };
                    spawn_text(tile, value.to_string(), font_size, color, fonts);
                }
            });
        });
}

fn spawn_moving_tiles(
    parent: &mut ChildSpawnerCommands,
    game: &Game,
    progress: f32,
    fonts: &GameFonts,
) {
    for motion in &game.motions {
        let from = cell_position(motion.from);
        let to = cell_position(motion.to);
        let position = from.lerp(to, progress);
        spawn_absolute_tile(parent, motion.value, position, motion.merged, fonts);
    }
}

fn spawn_absolute_tile(
    parent: &mut ChildSpawnerCommands,
    value: u32,
    position: Vec2,
    merged: bool,
    fonts: &GameFonts,
) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                width: Val::Px(TILE_SIZE),
                height: Val::Px(TILE_SIZE),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(tile_color(value)),
            moving_tile_shadow(merged),
        ))
        .with_children(|tile| {
            let font_size = match value.to_string().len() {
                1 => 62.0,
                2 => 60.0,
                3 => 54.0,
                4 => 48.0,
                _ => 42.0,
            };
            let color = if value <= 4 {
                color_ink()
            } else {
                color_light_text()
            };
            spawn_text(tile, value.to_string(), font_size, color, fonts);
        });
}

fn spawn_game_over_modal(parent: &mut ChildSpawnerCommands, fonts: &GameFonts) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.18, 0.16, 0.14, 0.58)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: Val::Px(430.0),
                        height: Val::Px(300.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(24.0),
                        padding: UiRect::all(Val::Px(28.0)),
                        border_radius: BorderRadius::all(Val::Px(34.0)),
                        ..default()
                    },
                    BackgroundColor(color_panel()),
                ))
                .with_children(|modal| {
                    spawn_text(modal, "Game Over", 52.0, color_ink(), fonts);
                    spawn_text(
                        modal,
                        "No more moves are available.",
                        22.0,
                        color_secondary_text(),
                        fonts,
                    );

                    modal
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(16.0),
                            ..default()
                        })
                        .with_children(|buttons| {
                            spawn_modal_button(buttons, "Restart", RestartButton, fonts);
                            spawn_modal_button(buttons, "Close Game", CloseButton, fonts);
                        });
                });
        });
}

fn spawn_modal_button<T: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    marker: T,
    fonts: &GameFonts,
) {
    parent
        .spawn((
            Button,
            marker,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(56.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(button_color()),
        ))
        .with_children(|button| {
            spawn_text(button, label, 21.0, color_light_text(), fonts);
        });
}

fn spawn_text(
    parent: &mut ChildSpawnerCommands,
    text: impl Into<String>,
    font_size: f32,
    color: Color,
    fonts: &GameFonts,
) {
    spawn_text_with_weight(parent, text, font_size, color, FontWeight::BOLD, fonts);
}

fn spawn_text_with_weight(
    parent: &mut ChildSpawnerCommands,
    text: impl Into<String>,
    font_size: f32,
    color: Color,
    weight: FontWeight,
    fonts: &GameFonts,
) {
    // All game text uses bundled Clear Sans font files for closer parity with the original 2048 UI.
    parent.spawn((
        Text::new(text),
        TextFont {
            font: FontSource::Handle(font_for_weight(fonts, weight)),
            font_size: FontSize::Px(font_size),
            weight,
            ..default()
        },
        TextColor(color),
    ));
}

fn font_for_weight(fonts: &GameFonts, weight: FontWeight) -> Handle<Font> {
    if weight >= FontWeight::BOLD {
        fonts.bold.clone()
    } else if weight >= FontWeight::MEDIUM {
        fonts.medium.clone()
    } else {
        fonts.regular.clone()
    }
}

fn tile_motion_progress(elapsed: f32) -> f32 {
    let progress = (elapsed / TILE_ANIMATION_SECONDS).clamp(0.0, 1.0);
    1.0 - (1.0 - progress).powi(4)
}

fn tile_pop_scale(game: &Game, row: usize, col: usize, elapsed: f32) -> f32 {
    let pop_elapsed = elapsed - TILE_ANIMATION_SECONDS;
    if !(0.0..=TILE_POP_SECONDS).contains(&pop_elapsed) {
        return 1.0;
    }

    let cell = (row, col);
    let was_merged = game
        .motions
        .iter()
        .any(|motion| motion.merged && motion.to == cell);

    if !was_merged {
        return 1.0;
    }

    // A tiny single pulse makes merges feel physical without adding visual wobble.
    let progress = pop_elapsed / TILE_POP_SECONDS;
    let pulse = 1.0 - (progress * 2.0 - 1.0).abs();
    1.0 + pulse * 0.045
}

fn cell_position((row, col): (usize, usize)) -> Vec2 {
    Vec2::new(
        TILE_GAP + col as f32 * (TILE_SIZE + TILE_GAP),
        TILE_GAP + row as f32 * (TILE_SIZE + TILE_GAP),
    )
}

fn should_hide_static_tile(game: &Game, row: usize, col: usize) -> bool {
    let cell = (row, col);
    game.spawned_cell == Some(cell) || game.motions.iter().any(|motion| motion.to == cell)
}

fn tile_shadow(value: u32) -> BoxShadow {
    if value == 0 {
        BoxShadow(Vec::new())
    } else {
        BoxShadow::new(
            Color::srgba(0.28, 0.22, 0.17, 0.26),
            Val::Px(0.0),
            Val::Px(6.0),
            Val::Px(-2.0),
            Val::Px(10.0),
        )
    }
}

fn moving_tile_shadow(merged: bool) -> BoxShadow {
    let opacity = if merged { 0.30 } else { 0.24 };
    BoxShadow::new(
        Color::srgba(0.28, 0.22, 0.17, opacity),
        Val::Px(0.0),
        Val::Px(6.0),
        Val::Px(-2.0),
        Val::Px(10.0),
    )
}

// Palette tuned from the reference: cream background, warm taupe board,
// pale empty cells, and the classic 2048 orange/yellow progression.
pub fn color_bg() -> Color {
    Color::srgb(0.99, 0.98, 0.94)
}

fn color_board() -> Color {
    Color::srgb(0.64, 0.57, 0.50)
}

fn color_panel() -> Color {
    Color::srgb(0.98, 0.94, 0.88)
}

fn color_ink() -> Color {
    Color::srgb(0.46, 0.39, 0.32)
}

fn color_title() -> Color {
    Color::srgb(0.46, 0.39, 0.32)
}

fn color_light_text() -> Color {
    Color::srgb(1.00, 0.98, 0.95)
}

fn color_secondary_text() -> Color {
    Color::srgb(0.56, 0.49, 0.42)
}

fn score_card_color() -> Color {
    Color::srgb(0.96, 0.94, 0.90)
}

fn score_card_border_color() -> Color {
    Color::srgb(0.91, 0.88, 0.81)
}

fn color_score_label() -> Color {
    Color::srgb(0.59, 0.52, 0.45)
}

fn color_score_value() -> Color {
    Color::srgb(0.54, 0.47, 0.40)
}

fn button_color() -> Color {
    Color::srgb(0.57, 0.50, 0.43)
}

fn button_hover_color() -> Color {
    Color::srgb(0.63, 0.56, 0.49)
}

fn button_pressed_color() -> Color {
    Color::srgb(0.45, 0.39, 0.33)
}

fn tile_color(value: u32) -> Color {
    match value {
        0 => Color::srgb(0.71, 0.64, 0.55),
        2 => Color::srgb(0.93, 0.89, 0.84),
        4 => Color::srgb(0.94, 0.86, 0.70),
        8 => Color::srgb(0.96, 0.70, 0.45),
        16 => Color::srgb(0.98, 0.55, 0.34),
        32 => Color::srgb(0.98, 0.44, 0.34),
        64 => Color::srgb(0.98, 0.32, 0.20),
        128 => Color::srgb(0.94, 0.82, 0.36),
        256 => Color::srgb(0.94, 0.79, 0.28),
        512 => Color::srgb(0.94, 0.76, 0.20),
        1024 => Color::srgb(0.94, 0.73, 0.13),
        2048 => Color::srgb(0.94, 0.70, 0.05),
        _ => Color::srgb(0.24, 0.22, 0.20),
    }
}
