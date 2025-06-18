use bevy::dev_tools::states::*;
use bevy::prelude::*;

const SPLASH_TIME: f32 = 2.0;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_TIME, TimerMode::Once))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Splash,
    GameSetup,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum InGame {
    #[default]
    Init,
    Running,
    Paused,
    GameOver,
}

impl std::fmt::Display for InGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InGame::Init => write!(f, "Init"),
            InGame::Running => write!(f, "Running"),
            InGame::Paused => write!(f, "Paused"), // Ensured correct typo here as well
            InGame::GameOver => write!(f, "Game Over"),
        }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<InGame>()
            .init_resource::<SplashTimer>()
            .add_systems(Update, countdown.run_if(in_state(AppState::Splash)))
            .add_systems(Update, log_transitions::<AppState>)
            .add_systems(Update, log_transitions::<InGame>);
    }
}

// transition splash to game
fn countdown(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::GameSetup);
    }
}
