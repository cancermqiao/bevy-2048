fn main() {
    bevy::prelude::App::new()
        .add_plugins(bevy_2048::Game2048Plugin)
        .run();
}
