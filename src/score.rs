use bevy::prelude::*;

use crate::{
    schedule::GameRunningSet,
    state::InGame,
    ui::game::{BestText, ScoreText},
};

#[derive(Resource, Default)]
pub struct ScoreRes {
    pub score: usize,
    pub best: usize,
}

impl ScoreRes {
    pub fn reset(&mut self) {
        self.score = 0;
    }
}

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScoreRes>()
            .add_systems(
                Update,
                (current_score_update, best_score_update).in_set(GameRunningSet::EntityUpdate),
            )
            .add_systems(OnEnter(InGame::Init), score_reset);
    }
}

fn current_score_update(mut query: Query<&mut Text, With<ScoreText>>, score_res: Res<ScoreRes>) {
    for mut text in query.iter_mut() {
        text.0 = score_res.score.to_string();
    }
}

fn best_score_update(mut query: Query<&mut Text, With<BestText>>, score_res: Res<ScoreRes>) {
    if score_res.score > score_res.best {
        for mut text in query.iter_mut() {
            text.0 = score_res.score.to_string();
        }
    }
}

fn score_reset(mut score_res: ResMut<ScoreRes>) {
    score_res.reset();
}
