use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::paddle::spawn_paddle;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board);
    }
}

#[derive(Component)]
pub struct Board;

impl Board {
    const COLOR: Color = Color::BLUE;
    const SIZE: Vec3 = Vec3 {
        x: 20.0,
        y: 0.2,
        z: 20.0,
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
) {
    commands
        .spawn((
            Name::new("BOARD"),
            Board,
            PbrBundle {
                mesh: meshes.add(Board::shape().into()),
                transform: Board::transform(),
                material: materials.add(Board::COLOR.into()),
                ..default()
            },
        ))
        .insert((RigidBody::Fixed, Board::collider()))
        .with_children(|builder| {
            let positions = [
                // Vec3::new(0.0, 0.0, 0.0),
                // Vec3::new(8.0, 0.0, 0.0),
                Vec3::new(-8.0, 0.0, 8.0),
                Vec3::new(-8.0, 0.0, -8.0),
                Vec3::new(8.0, 0.0, -8.0),
                Vec3::new(8.0, 0.0, 8.0),
            ];
            for pos in positions {
                spawn_paddle(builder, pos)
            }
        });
}
