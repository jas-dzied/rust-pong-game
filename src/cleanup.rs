use bevy::prelude::*;

use crate::control::{Ball, LivesCounter, GlobalState};

#[derive(Component)]
pub struct HitCounter {
    pub count: i32,
}
#[derive(Component)]
pub struct DestructionTimer {
    pub timer: Timer
}

pub struct CleanupPlugin;
impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(check_timers)
            .add_system(collect_fallen_balls);
    }
}

fn collect_fallen_balls(
    balls: Query<(Entity, &Ball, &Transform)>,
    mut state: ResMut<GlobalState>,
    mut counters: Query<(&mut LivesCounter, &mut Text)>,
    mut commands: Commands,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let (mut counter, mut counter_text) = counters.single_mut();
    for (e, _, transform) in balls.iter() {
        if transform.translation.y < -(window.height()/2.0+80.0) {
            commands.entity(e).despawn();
            counter.count -= 1;
            counter_text.sections[0].value = format!("Lives: {}", counter.count);
            if counter.count <= 0 {
                state.lost = true;
                println!("LOST");
            }
        }
    }
}

fn check_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut objects: Query<(Entity, &mut DestructionTimer)>
) {
    for (e, mut timer) in objects.iter_mut() {
        if timer.timer.tick(time.delta()).just_finished() {
            commands.entity(e).despawn();
        }
    }
}
