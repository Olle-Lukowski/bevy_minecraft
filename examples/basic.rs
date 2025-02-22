use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_minecraft::MinecraftPlugin;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            LogPlugin {
                level: Level::INFO,
                ..default()
            },
            MinecraftPlugin::default(),
        ))
        .run();
}
