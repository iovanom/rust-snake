use bevy::{
    prelude::*,
    window::{close_on_esc, PresentMode},
};
use rust_snake::{controls::ControlsPlugin, game::GamePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "rust Snake!".to_string(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(ControlsPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
