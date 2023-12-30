use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod ball;
mod camera;

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
        .add_plugins((
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((camera::CameraPlugin, ball::BallPlugin))
        // STARTUP
        // .add_startup_system(load_font)
        .add_systems(Startup, (spawn_light, spawn_ground))
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

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground plane
    commands
        .spawn((
            Name::new("Ground"),
            PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(50.0).into()),
                material: materials.add(Color::SILVER.into()),
                ..default()
            },
        ))
        .insert((RigidBody::Fixed, Collider::cuboid(25., 0., 25.)));
}
