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
    mut counters: Query<(Entity, &mut LivesCounter, &mut Text)>,
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>
) {
    if state.lost == false {
        let window = windows.get_primary().unwrap();
        let (counter_e, mut counter, mut counter_text) = counters.single_mut();
        for (e, _, transform) in balls.iter() {
            if transform.translation.y < -(window.height()/2.0+80.0) {
                commands.entity(e).despawn();
                counter.count -= 1;
                counter_text.sections[0].value = format!("Lives: {}", counter.count);
                if counter.count <= 0 {
                    state.lost = true;
                    commands.entity(counter_e).despawn();

                    let font = asset_server.load("ka1.ttf");
                    let text_style = TextStyle {
                        font: font.clone(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    };
                    let text_alignment = TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    };
                    commands
                        .spawn_bundle(Text2dBundle {
                            text: Text::with_section("YOU LOST!", text_style.clone(), text_alignment),
                            transform: Transform {
                                translation: Vec3::new(0.0, 0.0, 0.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(LivesCounter {
                            count: 5,
                        });

                }
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
