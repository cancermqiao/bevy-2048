use bevy::prelude::*;

use crate::{asset_loader::fonts::FontAsset, state::InGame, ui::{NewGameButton, TEXT_COLOR}};


#[derive(Component)]
struct PausedMenu;

#[derive(Component)]
pub struct CancelButton;

pub struct PausedMenuPlugin;

impl Plugin for PausedMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGame::Paused), spawn_paused_menu);

    }
}

fn spawn_paused_menu(mut commands: Commands, font_asset: Res<FontAsset>) {
    commands
        .spawn((
            StateScoped(InGame::Paused),
            PausedMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(24.0),
                            padding: UiRect::all(Val::Px(40.0)),
                            ..default()
                    },
                    BackgroundColor(Color::srgb_u8(250, 248, 239)),
                    BorderRadius::all(Val::Px(12.0)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("New Game"),
                        TextFont {
                            font: font_asset.bold.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));

                    // Text
                    parent.spawn((
                        Text::new("Are you sure you want to start a new game?"),
                        TextFont {
                            font: font_asset.regular.clone(),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));

                    // Text
                    parent.spawn((
                        Text::new("All your progress will be lost."),
                        TextFont {
                            font: font_asset.regular.clone(),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));

                    // Buttons container
                    parent
                        .spawn(Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Stretch, // Stretch buttons
                                row_gap: Val::Px(16.0),
                                margin: UiRect::top(Val::Px(20.0)),
                                width: Val::Percent(100.0),
                                ..default()
                        })
                        .with_children(|parent| {
                            // New Game Button (Primary)
                            parent.spawn((
                                Button,
                                NewGameButton,
                                Node {
                                        padding: UiRect::all(Val::Px(16.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        width: Val::Percent(100.0),
                                        ..default()
                                },
                                BackgroundColor(Color::srgb_u8(148, 133, 118)),
                                BorderRadius::all(Val::Px(8.0)),
                                BorderColor(Color::srgb_u8(0, 116, 217)),
                                children![(
                                    Text::new("Start New Game"),
                                    TextFont {
                                        font: font_asset.semi_bold.clone(),
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                )],
                            ));

                            // Cancel Button (Secondary)
                            parent.spawn((
                                Button,
                                CancelButton,
                                Node {
                                        padding: UiRect::all(Val::Px(16.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        width: Val::Percent(100.0),
                                        ..default()
                                },
                                BorderRadius::all(Val::Px(8.0)),
                                BorderColor(Color::srgb_u8(148, 133, 118)),
                                children![(
                                    Text::new("Cancel"),
                                    TextFont {
                                        font: font_asset.semi_bold.clone(),
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(TEXT_COLOR),
                                )],
                            ));
                        });
                });
        });
}
