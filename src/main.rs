use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use paddle::Paddle;

mod ball;
mod board;
mod camera;
mod paddle;

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
        .add_plugins(DebugPlugin)
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

struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(1.0))
            .add_plugins((
                bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(Startup, spawn_debug_ui)
            .add_systems(FixedUpdate, debug)
            .add_systems(Update, update_debug_ui);
    }
}

#[derive(Component)]
struct DebugUi;

#[allow(dead_code, unused_variables)]
fn debug(
    q_paddles: Query<&Transform, With<Paddle>>,
    q_joint: Query<&ImpulseJoint>,
    q_collider: Query<&Collider, With<Paddle>>,
) {
    // for paddle_transform in q_paddles.iter() {
    //     info!("DEBUG paddle pos: {paddle_transform:?}");
    // }
    // if let Ok(joint) = q_joint.get_single() {
    //     info!("DEBUG joint pos: {joint:?}");
    // }
    // if let Ok(collider) = q_collider.get_single() {
    //     info!("DEBUG joint pos: {collider:?}");
    // }
}

fn spawn_debug_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                DebugUi,
                TextBundle::from_section(
                    "Text Example",
                    TextStyle {
                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 10.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
        });
}

#[allow(dead_code, unused_variables)]
fn update_debug_ui(
    mut q_txt: Query<&mut Text, With<DebugUi>>,
    q_paddles: Query<&Transform, With<Paddle>>,
) {
    if let Ok(mut txt) = q_txt.get_single_mut() {
        let mut line = String::new();
        for trans in q_paddles.iter() {
            line.push_str(format!("Transform: {trans:?} - ").as_str());
        }
        line.pop();
        line.pop();
        txt.sections[0].value = line;
    }
}
