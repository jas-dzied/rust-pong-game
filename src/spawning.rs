use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::physics::{Velocity, Accel};
use crate::control::{Ball};
use crate::cleanup::{HitCounter, DestructionTimer};

pub struct SpawnTimer(pub Timer);

pub struct SpawningPlugin;
impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_squares);
    }
}

fn spawn_squares(
    windows: Res<Windows>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let mut rng = thread_rng();
    let window = windows.get_primary().unwrap();
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn_simple_ball(
            &asset_server,
            Vec3::new(0.0, window.height()/2.0-100.0, 0.0),
            Velocity {
                x: rng.gen_range(-10.0..10.0),
                y: -10.0,
            }
        );
        commands.spawn_gravity_ball(
            &asset_server,
            Vec3::new(0.0, window.height()/2.0-100.0, 0.0),
            Velocity {
                x: rng.gen_range(-3.0..3.0),
                y: -10.0,
            },
            Accel {
                x: 0.0,
                y: -0.5,
            }
        );
    }
}

pub trait ExplosionSpawner {
    fn spawn_explosion(&mut self, asset_server: &Res<AssetServer>, position: Vec3) -> &mut Self;
}

impl ExplosionSpawner for Commands<'_, '_> {
    fn spawn_explosion<'a>(
        &mut self,
        asset_server: &Res<AssetServer>,
        position: Vec3,
    ) -> &mut Self {
        self
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("explosion.png"),
                transform: Transform {
                    translation: position,
                    scale: Vec3::new(3.0, 3.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(DestructionTimer {
                timer: Timer::from_seconds(1.0, false)
            });
        self
    }
}

pub trait BallSpawner {
    fn spawn_simple_ball(&mut self, asset_server: &Res<AssetServer>, position: Vec3, velocity: Velocity) -> &mut Self;
    fn spawn_gravity_ball(&mut self, asset_server: &Res<AssetServer>, position: Vec3, velocity: Velocity, accel: Accel) -> &mut Self;
}

impl BallSpawner for Commands<'_, '_> {

    fn spawn_simple_ball<'a>(
        &mut self,
        asset_server: &Res<AssetServer>,
        position: Vec3,
        velocity: Velocity,
    ) -> &mut Self {
        self
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("ball.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(velocity)
            .insert(Ball)
            .insert(HitCounter{
                count: 3,
            });
        self
    }

    fn spawn_gravity_ball<'a>(
        &mut self,
        asset_server: &Res<AssetServer>,
        position: Vec3,
        velocity: Velocity,
        accel: Accel,
    ) -> &mut Self {
        self
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("gravity-ball.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(velocity)
            .insert(accel)
            .insert(Ball)
            .insert(HitCounter{
                count: 0,
            });
        self
    }

}
