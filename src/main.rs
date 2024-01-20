use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod ball;
mod board;
mod camera;
mod config;
mod debug;
mod paddle;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States, Default)]
enum GameState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Default, Resource)]
struct GameAssets {
    scene: Handle<Scene>,
    colliders: Vec<(Collider, Transform)>,
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    LoadGltfError,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BEVY Pinball".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_state::<GameState>()
        .insert_resource(GameAssets::default())
        .add_plugins(debug::DebugPlugin)
        .add_plugins((
            camera::CameraPlugin,
            board::BoardPlugin,
            ball::BallPlugin,
            paddle::PaddlePlugin,
        ))
        // STARTUP
        // .add_startup_system(load_font)
        .add_systems(Startup, spawn_light)
        .run();
}

// fn load_font(mut commands: Commands, server: Res<AssetServer>) {
//     let handle: Handle<Font> = server.load("fonts/FiraSans-Bold.ttf");
//     commands.insert_resource(UiFont(handle));
// }

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}
