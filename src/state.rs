use bevy::{app::AppExit, prelude::*};

use crate::{
    asset_loader::ButtonAssets,
    game::Game,
    tiles::Position,
    ui::{ExitButton, PauseButton, RepeatButton},
};

#[derive(Debug, Default, States, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnExit(GameState::GameOver), game_reset)
            .add_systems(
                Update,
                (
                    pause_button_interaction_system.run_if(not(in_state(GameState::GameOver))),
                    repeat_button_interaction_system,
                    exit_button_interaction_system,
                    transition_to_in_game.run_if(in_state(GameState::GameOver)),
                ),
            );
    }
}

fn pause_button_interaction_system(
    mut pause_button: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<PauseButton>),
    >,
    button_assets: Res<ButtonAssets>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((interaction, mut ui_image)) = pause_button.get_single_mut() else {
        return;
    };
    ui_image.texture = match interaction {
        Interaction::Pressed => match state.get() {
            GameState::Playing => {
                next_state.set(GameState::Paused);
                button_assets.play.hover.clone()
            }
            GameState::Paused => {
                next_state.set(GameState::Playing);
                button_assets.pause.hover.clone()
            }
            GameState::GameOver => ui_image.texture.clone(),
        },
        Interaction::Hovered => match state.get() {
            GameState::Playing => button_assets.pause.hover.clone(),
            GameState::Paused => button_assets.play.hover.clone(),
            GameState::GameOver => ui_image.texture.clone(),
        },
        Interaction::None => match state.get() {
            GameState::Playing => button_assets.pause.idle.clone(),
            GameState::Paused => button_assets.play.idle.clone(),
            GameState::GameOver => ui_image.texture.clone(),
        },
    }
}

fn repeat_button_interaction_system(
    mut repeat_button: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<RepeatButton>),
    >,
    button_assets: Res<ButtonAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((interaction, mut ui_image)) = repeat_button.get_single_mut() else {
        return;
    };
    ui_image.texture = match interaction {
        Interaction::Pressed => {
            next_state.set(GameState::GameOver);
            button_assets.repeat.hover.clone()
        }
        Interaction::Hovered => button_assets.repeat.hover.clone(),
        Interaction::None => button_assets.repeat.idle.clone(),
    }
}

fn exit_button_interaction_system(
    mut exit_button: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<ExitButton>),
    >,
    button_assets: Res<ButtonAssets>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let Ok((interaction, mut ui_image)) = exit_button.get_single_mut() else {
        return;
    };
    ui_image.texture = match interaction {
        Interaction::Pressed => {
            app_exit_events.send(AppExit);
            button_assets.exit.hover.clone()
        }
        Interaction::Hovered => button_assets.exit.hover.clone(),
        Interaction::None => button_assets.exit.idle.clone(),
    }
}

fn transition_to_in_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
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
