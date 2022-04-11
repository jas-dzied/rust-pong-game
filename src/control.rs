use bevy::prelude::*;

const PLAYER_SPEED: f32 = 25.0;

#[derive(Component)]
pub struct Player {
    pub lives: i32,
}
#[derive(Component)]
pub struct Ball;
#[derive(Component)]
pub struct LivesCounter{
    pub count: i32,
}
#[derive(Component)]
pub struct ScoreCounter{
    pub count: i32,
}
pub struct GlobalState {
    pub lost: bool,
}

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(player_movement);
    }
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut players: Query<(&mut Transform, With<Player>)>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    for (mut transform, _) in players.iter_mut() {
        if keyboard.pressed(KeyCode::A) && transform.translation.x > -(window.width()/2.0)+96.0 {
            transform.translation.x -= PLAYER_SPEED;
        }
        if keyboard.pressed(KeyCode::D) && transform.translation.x < window.width()/2.0-96.0 {
            transform.translation.x += PLAYER_SPEED;
        }
    }
}
