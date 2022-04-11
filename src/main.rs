use bevy::prelude::*;

mod control;
use crate::control::{ControlPlugin, GlobalState};

mod physics;
use crate::physics::{PhysicsPlugin};

mod spawning;
use crate::spawning::{SpawningPlugin, SpawnTimer};

mod cleanup;
use crate::cleanup::{CleanupPlugin};

mod startup;
use crate::startup::{StartupPlugin};

fn main() {
    App::new()

        .add_plugins(DefaultPlugins)

        .insert_resource(SpawnTimer(Timer::from_seconds(3.0, true)))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(GlobalState {
                lost: false,
        })

        .add_plugin(StartupPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(SpawningPlugin)
        .add_plugin(ControlPlugin)
        .add_plugin(CleanupPlugin)

        .run();
}
