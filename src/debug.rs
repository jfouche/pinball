use bevy::{gltf::GltfExtras, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{config, paddle::Paddle, GameAssets, GameState};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(1.0))
            .add_plugins((
                bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(OnEnter(GameState::Loading), (start_assets_loading,))
            .add_systems(
                Update,
                (check_if_loaded,).run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnEnter(GameState::Loaded), (debug_gltf,))
            .add_systems(Startup, (spawn_debug_ui, init_debug))
            .add_systems(FixedUpdate, debug)
            .add_systems(Update, update_debug_ui);
    }
}

#[derive(Component)]
struct DebugUi;

fn init_debug() {
    config::test();
}

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

// load the scene from the gltf file
fn start_assets_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        scene: asset_server.load("pinball.glb#Scene0"),
        ..default()
    });
}

// check if the scene is loaded and if so, get the colliders from it
fn check_if_loaded(
    scenes: Res<Assets<Scene>>,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if scenes.get(&game_assets.scene).is_some() {
        game_state.set(GameState::Loaded);
    }
}

fn debug_gltf(game_assets: Res<GameAssets>, mut scenes: ResMut<Assets<Scene>>) {
    pub fn debug(world: &mut World) {
        let mut extras_q = world.query::<&GltfExtras>();
        for extras in extras_q.iter(world) {
            info!("get_scene_colliders - extras: {extras:?}")
        }

        let mut meshes_q = world.query::<(Entity, &Name, Option<&Children>)>();
        for (entity, entity_name, children) in meshes_q.iter(world) {
            info!("get_scene_colliders - entity: {entity:?} ({entity_name:?})");
            if let Some(children) = children {
                for e in children {
                    info!("     child: {e:?}");
                }
            }
        }
    }
    if let Some(scene) = scenes.get_mut(&game_assets.scene) {
        debug(&mut scene.world)
    }
}
