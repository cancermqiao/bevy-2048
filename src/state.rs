use bevy::prelude::*;

use crate::{color, game::Game, tiles::Position};

#[derive(Debug, Default, States, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Playing), game_reset)
            .add_systems(Update, (button_interaction_system, button_text_system));
    }
}

fn button_interaction_system(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in query.iter_mut() {
        *color = match interaction {
            Interaction::Pressed => {
                match state.get() {
                    GameState::Playing => {
                        next_state.set(GameState::GameOver);
                    }
                    GameState::GameOver => {
                        next_state.set(GameState::Playing);
                    }
                }
                color::button::PRESSED
            }
            Interaction::Hovered => color::button::HOVERED,
            Interaction::None => color::button::NORMAL,
        }
        .into();
    }
}

fn button_text_system(
    button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
    state: Res<State<GameState>>,
) {
    let children = button_query.single();
    let first_child_entity = children
        .first()
        .expect("expect button to have a first child");
    let mut text = text_query
        .get_mut(*first_child_entity)
        .expect("expected Text to exist");
    let text_section = text
        .sections
        .first_mut()
        .expect("expected first section to be accessible as mutable");
    match state.get() {
        GameState::Playing => {
            text_section.value = "End Game".to_string();
        }
        GameState::GameOver => {
            text_section.value = "New Game".to_string();
        }
    }
}

fn game_reset(
    mut commands: Commands,
    tiles: Query<Entity, With<Position>>,
    mut game: ResMut<Game>,
) {
    for entity in tiles.iter() {
        commands.entity(entity).despawn_recursive();
    }
    if game.best_score < game.score {
        game.best_score = game.score;
    }
    game.score = 0;
}
