use bevy::prelude::*;

use crate::control::{Player, Ball, ScoreCounter, GlobalState};
use crate::cleanup::{HitCounter, DestructionTimer};
use crate::spawning::ExplosionSpawner;

#[derive(Debug, Component, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
#[derive(Component, Clone, Copy, PartialEq)]
pub struct Accel {
    pub x: f32,
    pub y: f32,
}
#[derive(Component, Clone, Copy, PartialEq)]
pub struct SpeedDown {
    pub amount: f32,
}
#[derive(Component)]
pub struct SpeedRotation;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(apply_velocity)
            .add_system(check_bounce)
            .add_system(apply_speed_down)
            .add_system(apply_speed_rotation)
            .add_system(apply_accel);
    }
}

fn apply_velocity(
    mut objects: Query<(&Velocity, &mut Transform)>
) {
    for (velocity, mut transform) in objects.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn apply_accel(
    mut objects: Query<(&Accel, &mut Velocity)>
) {
    for (accel, mut velocity) in objects.iter_mut() {
        velocity.x += accel.x;
        velocity.y += accel.y;
    }
}

fn apply_speed_down(
    mut objects: Query<(&SpeedDown, &mut Velocity)>
) {
    for (speed_down, mut velocity) in objects.iter_mut() {
        velocity.x *= speed_down.amount;
        velocity.y *= speed_down.amount;
    }
}

fn apply_speed_rotation(
    mut objects: Query<(&Velocity, &mut Transform, With<SpeedRotation>)>
) {
    for (velocity, mut transform, _) in objects.iter_mut() {
        transform.rotation = transform.rotation.mul_quat(Quat::from_rotation_z(0.15*(velocity.x+velocity.y).abs()));
    }
}

fn check_bounce(
    state: Res<GlobalState>,
    mut commands: Commands,
    windows: Res<Windows>,
    mut q: QuerySet<(
        QueryState<(Entity, &Ball, &mut Transform, &mut Velocity, &mut HitCounter, &mut Handle<Image>)>,
        QueryState<(&Player, &Transform)>
    )>,
    mut counters: Query<(&mut ScoreCounter, &mut Text)>,
    asset_server: Res<AssetServer>
) {
    if state.lost == false {
        let (mut counter, mut counter_text) = counters.single_mut();
        let window = windows.get_primary().unwrap();
        let (_, paddle) = q.q1().single();
        let paddle_translation = paddle.translation.clone();
        for (e, _, mut transform, mut velocity, mut hit_counter, mut texture) in q.q0().iter_mut() {

            let ball_lower_edge = transform.translation.y-16.0;
            let window_half = window.height()/2.0;

            if ball_lower_edge < -(window_half)+59.5 && transform.translation.y > -(window_half+80.0) {
                let paddle_left = paddle_translation.x - 96.0;
                let paddle_right = paddle_translation.x + 96.0;
                if transform.translation.x < paddle_right && transform.translation.x > paddle_left {
                    velocity.y *= -1.0;
                    velocity.y -= 5.0;
                    transform.translation.y = -(window_half - 16.0)+59.5;

                    if hit_counter.count < 3 {
                        hit_counter.count += 1;
                    } else {
                        commands.entity(e).despawn();
                        commands.spawn_explosion(transform.translation);
                        counter.count += 1;
                        counter_text.sections[0].value = format!("Score: {}", counter.count);
                    }
                }
            }

            if transform.translation.x-16.0 < -(window.width()/2.0) || transform.translation.x+16.0 > window.width()/2.0 {
                velocity.x *= -1.0;
            }
        }
    }
}
