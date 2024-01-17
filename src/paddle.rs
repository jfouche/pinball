use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::PaddleConfig;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_paddle);
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum PaddleType {
    Left,
    Right,
}

#[derive(Clone, Component)]
pub struct Paddle {
    ptype: PaddleType,
}

impl Paddle {
    pub const SIZE: Vec3 = Vec3::new(5.0, 0.5, 2.0);
    /// space between Paddle and board
    pub const SPACE: f32 = 0.5;

    pub fn hx() -> f32 {
        Self::SIZE.x / 2.0
    }

    pub fn hy() -> f32 {
        Self::SIZE.y / 2.0
    }

    pub fn hz() -> f32 {
        Self::SIZE.z / 2.0
    }

    fn shape() -> impl Into<Mesh> {
        shape::Box::new(Self::SIZE.x, Self::SIZE.y, Self::SIZE.z)
    }

    fn collider() -> Collider {
        Collider::cuboid(Self::hx(), Self::hy(), Self::hz())
    }
}

pub fn spawn_paddle(
    builder: &mut ChildBuilder,
    config: &PaddleConfig,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let board_entity = builder.parent_entity();
    builder
        .spawn((
            Name::new("PADDLE"),
            Paddle {
                ptype: config.ptype,
            },
            PbrBundle {
                mesh: meshes.add(Paddle::shape().into()),
                transform: config.transform(),
                material: materials.add(Color::BLUE.into()),
                ..default()
            },
            // Paddle::transform(&pos),
        ))
        .insert((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            Paddle::collider(),
            ColliderMassProperties::Mass(10.0),
            ImpulseJoint::new(board_entity, config.joint()),
        ));
}

fn move_paddle(mut q_paddles: Query<(&Paddle, &mut ImpulseJoint)>, keys: Res<Input<KeyCode>>) {
    let left = keys.pressed(KeyCode::Left);
    let right = keys.pressed(KeyCode::Right);

    let stiffness = 5000.0;
    let damping = 1.0;
    for (paddle, mut impulse_joint) in q_paddles.iter_mut() {
        if let Some(joint) = impulse_joint.data.as_revolute_mut() {
            let target_pos = match paddle.ptype {
                PaddleType::Left if left => 0.4,
                PaddleType::Left => -0.4,
                PaddleType::Right if right => -0.4,
                PaddleType::Right => 0.4,
            };
            joint.set_motor_position(target_pos, stiffness, damping);
        }
    }
}
