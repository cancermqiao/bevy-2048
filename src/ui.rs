use bevy::prelude::*;

use crate::{
    asset_loader::{ButtonAssets, FontSpec},
    color::{
        score::{SCORE_BOARD_PADDING, SCORE_BOX, SCORE_FONT_SIZE},
        tile::{TILE_TEXT_COLOR_DARK, TILE_TEXT_COLOR_LIGHT},
    },
};
#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

#[derive(Component)]
pub struct PauseButton;

#[derive(Component)]
pub struct RepeatButton;

#[derive(Component)]
pub struct ExitButton;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
    }
}

fn setup_ui(mut commands: Commands, font_spec: Res<FontSpec>, button_assets: Res<ButtonAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                top: Val::Percent(5.0),
                column_gap: Val::Percent(2.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // colum1
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        justify_content: JustifyContent::Center,
                        top: Val::Percent(3.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "2048",
                        TextStyle {
                            font: font_spec.family.clone(),
                            font_size: 60.0,
                            color: TILE_TEXT_COLOR_DARK,
                        },
                    ));
                });
            // colum2
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        row_gap: Val::Px(20.),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // score board
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // scorebox
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        padding: SCORE_BOARD_PADDING,
                                        row_gap: Val::Px(10.0),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(SCORE_BOX),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Score",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: SCORE_FONT_SIZE,
                                            color: TILE_TEXT_COLOR_LIGHT,
                                        },
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "<score>",
                                            TextStyle {
                                                font: font_spec.family.clone(),
                                                font_size: SCORE_FONT_SIZE,
                                                color: TILE_TEXT_COLOR_LIGHT,
                                            },
                                        ),
                                        ScoreDisplay,
                                    ));
                                });
                            // end scorebox
                            // best scorebox
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        padding: SCORE_BOARD_PADDING,
                                        row_gap: Val::Px(10.0),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(SCORE_BOX),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Best",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: SCORE_FONT_SIZE,
                                            color: TILE_TEXT_COLOR_LIGHT,
                                        },
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "<score>",
                                            TextStyle {
                                                font: font_spec.family.clone(),
                                                font_size: SCORE_FONT_SIZE,
                                                color: TILE_TEXT_COLOR_LIGHT,
                                            },
                                        ),
                                        BestScoreDisplay,
                                    ));
                                });
                            // end best scorebox
                        });
                    // button
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // repeat
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(50.0),
                                        height: Val::Px(50.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: UiImage::new(button_assets.repeat.idle.clone()),
                                    ..default()
                                },
                                RepeatButton,
                            ));
                            // paused
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(50.0),
                                        height: Val::Px(50.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: UiImage::new(button_assets.pause.idle.clone()),
                                    ..default()
                                },
                                PauseButton,
                            ));
                            // exit
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(50.0),
                                        height: Val::Px(50.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: UiImage::new(button_assets.exit.idle.clone()),
                                    ..default()
                                },
                                ExitButton,
                            ));
                        });
                });
        });
}
