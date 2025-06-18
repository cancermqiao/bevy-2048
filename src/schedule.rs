use bevy::prelude::*;

use crate::state::InGame;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameRunningSet {
    UserInput,
    EntityUpdate,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (GameRunningSet::UserInput, GameRunningSet::EntityUpdate)
                .chain()
                .run_if(in_state(InGame::Running)),
        );
    }
}
