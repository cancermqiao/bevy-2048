use bevy::prelude::*;

use crate::{asset_loader::FontSpec, color::score::SCORE_BOX};

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
    }
}

fn setup_ui(mut commands: Commands, font_spec: Res<FontSpec>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "2048",
                TextStyle {
                    font: font_spec.family.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // scorebox
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                padding: UiRect {
                                    left: Val::Px(20.0),
                                    right: Val::Px(20.0),
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
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
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ));
                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
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
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                padding: UiRect {
                                    left: Val::Px(20.0),
                                    right: Val::Px(20.0),
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
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
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ));
                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                BestScoreDisplay,
                            ));
                        });
                    // end best scorebox
                });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(130.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Button",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..default()
                    });
                });
        });
}
