use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    config::{self, PinballConfig},
    paddle::spawn_paddle,
    GameAssets, GameState,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(config::load_config())
            .add_systems(OnEnter(GameState::Loading), (load_scene,))
            .add_systems(
                Update,
                (check_if_loaded,).run_if(in_state(GameState::Loading)),
            )
            // .add_systems(OnEnter(GameState::Loaded), (spawn_gltf,))
            .add_systems(Startup, spawn_board);
    }
}

#[derive(Component)]
pub struct Board;

impl Board {
    const COLOR: Color = Color::PURPLE;
    const SIZE: Vec3 = Vec3 {
        x: 40.0,
        y: 0.2,
        z: 40.0,
    };

    fn hx() -> f32 {
        Self::SIZE.x / 2.0
    }

    fn hy() -> f32 {
        Self::SIZE.y / 2.0
    }

    fn hz() -> f32 {
        Self::SIZE.z / 2.0
    }

    fn transform() -> Transform {
        Transform::from_xyz(0.0, -Self::hy(), 0.0)
    }

    fn shape() -> impl Into<Mesh> {
        shape::Box::new(Self::SIZE.x, Self::SIZE.y, Self::SIZE.z)
    }

    fn collider() -> Collider {
        Collider::cuboid(Self::hx(), Self::hy(), Self::hz())
    }
}

fn spawn_board(
    mut commands: Commands,
    // ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pinball_config: Res<PinballConfig>,
) {
    commands
        .spawn((
            Name::new("BOARD"),
            Board,
            PbrBundle {
                mesh: meshes.add(Board::shape().into()),
                transform: Board::transform()
                    .with_rotation(Quat::from_rotation_x(pinball_config.board.angle)),
                material: materials.add(Board::COLOR.into()),
                ..default()
            },
        ))
        .insert((RigidBody::Fixed, Board::collider()))
        .with_children(|builder| {
            for pcfg in pinball_config.paddles.iter() {
                spawn_paddle(builder, pcfg, &mut meshes, &mut materials);
            }
        });
}

// load the scene from the gltf file
fn load_scene(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAssets>) {
    game_assets.scene = asset_server.load("pinball.glb#Scene0");
}

// check if the scene is loaded and if so, get the colliders from it
fn check_if_loaded(
    mut scenes: ResMut<Assets<Scene>>,
    mut game_assets: ResMut<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Some(scene) = scenes.get_mut(&game_assets.scene) {
        // get_scene_colliders should be called only once per scene as it will remove the colliders meshes from it
        // game_assets.colliders =
        //     get_scene_colliders(&mut meshes, &mut scene.world).expect("Failed to create colliders");

        game_state.set(GameState::Loaded);
    }
}
