use bevy::prelude::*;

use crate::{
    state::{AppState, InGame},
    ui::{NewGameButton, paused_menu::CancelButton},
};

pub struct ButtonPlugins;

impl Plugin for ButtonPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (new_game_button_interaction, cancel_button_interaction)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn new_game_button_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
    state: Res<State<InGame>>,
    mut next_state: ResMut<NextState<InGame>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match state.get() {
                InGame::GameOver => {
                    next_state.set(InGame::Init);
                }
                InGame::Running => {
                    next_state.set(InGame::Paused);
                }
                InGame::Paused => {
                    next_state.set(InGame::Init);
                }
                _ => {}
            }
        }
    }
}

fn cancel_button_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<CancelButton>)>,
    mut next_state: ResMut<NextState<InGame>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(InGame::Running);
        }
    }
}