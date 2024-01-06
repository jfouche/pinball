use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board);
    }
}

#[derive(Component)]
pub struct Board;

const BOARD_SIZE: Vec3 = Vec3 {
    x: 15.0,
    y: 0.2,
    z: 30.0,
};

const BOARD_COLOR: Color = Color::BLUE;

fn spawn_board(mut commands: Commands, ass: Res<AssetServer>) {
    let my_gltf = ass.load("pinball.glb#Scene0");
    commands.spawn((
        Board,
        Name::new("Board"),
        SceneBundle {
            scene: my_gltf,
            transform: Transform::from_xyz(2.0, 0.0, -5.0),
            ..Default::default()
        },
    ));
}
