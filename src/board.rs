use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    config::{self, PinballConfig},
    paddle::spawn_paddle,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(config::load_config())
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
