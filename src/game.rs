use bevy::prelude::*;

use crate::ui::{BestScoreDisplay, ScoreDisplay};

#[derive(Default, Resource)]
pub struct Game {
    pub score: u32,
    pub best_score: u32,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>().add_systems(Update, scoreboard);
    }
}

fn scoreboard(
    game: Res<Game>,
    mut set: ParamSet<(
        Query<&mut Text, With<ScoreDisplay>>,
        Query<&mut Text, With<BestScoreDisplay>>,
    )>,
) {
    let mut score = set.p0();
    score.single_mut().sections[0].value = game.score.to_string();

    let mut best_score = set.p1();
    best_score.single_mut().sections[0].value = game.best_score.to_string();
}
