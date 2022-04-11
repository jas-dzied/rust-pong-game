use bevy::prelude::*;

use crate::control::{Player, LivesCounter, ScoreCounter};

pub struct GameFont(pub Handle<Font>);

pub struct StartupPlugin;
impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(startup);
    }
}

fn startup(
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    //fonts: Res<GameFont>,
    mut commands: Commands
) {

    let window = windows.get_primary().unwrap();
    let font = asset_server.load("ka1.ttf");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("paddle.png"),
            transform: Transform {
                scale: Vec3::new(3.0, 3.0, 1.0),
                translation: Vec3::new(0.0, -(window.height()/2.0)+40.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            lives: 10,
        });

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
            text: Text::with_section("Lives: 5", text_style.clone(), text_alignment),
            ..Default::default()
        })
        .insert(LivesCounter {
            count: 5,
        });
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("Score: 0", text_style.clone(), text_alignment),
            transform: Transform {
                translation: Vec3::new(0.0, 100.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreCounter {
            count: 0,
        });
    commands.insert_resource(GameFont(font));
}
