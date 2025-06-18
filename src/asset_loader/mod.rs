pub mod fonts;
pub mod icons;

use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use fonts::FontPlugin;
use icons::IconPlugin;

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone, Copy, Default)]
struct AssetLoadSchedule;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        let asset_load_schedule = Schedule::new(AssetLoadSchedule);
        app.add_schedule(asset_load_schedule);
        let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
        main_schedule_order.insert_startup_before(StateTransition, AssetLoadSchedule);

        app.add_plugins((IconPlugin, FontPlugin));
    }
}
