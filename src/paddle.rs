use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::Ground;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_paddle)
            // .insert_resource(Time::<Fixed>::from_seconds(1.0))
            // .add_systems(FixedUpdate, debug)
            // .add_systems(Update, (set_motor_pos))
            .add_systems(Update, (move_paddle,));
    }
}

// space between Paddle and board
const SPACE: f32 = 2.5;

#[derive(Component)]
pub struct Paddle;

impl Paddle {
    const PADDLE_SIZE: Vec3 = Vec3::new(5.0, 0.5, 2.0);

    const fn x() -> f32 {
        Self::PADDLE_SIZE.x
    }

    const fn y() -> f32 {
        Self::PADDLE_SIZE.y
    }

    const fn z() -> f32 {
        Self::PADDLE_SIZE.z
    }

    fn hx() -> f32 {
        Self::x() / 2.0
    }

    fn hy() -> f32 {
        Self::y() / 2.0
    }

    fn hz() -> f32 {
        Self::z() / 2.0
    }

    fn shape() -> shape::Box {
        shape::Box::new(Self::x(), Self::y(), Self::z())
    }

    fn collider() -> Collider {
        Collider::cuboid(Self::hx(), Self::hy(), Self::hz())
    }

    fn joint(height: f32) -> GenericJoint {
        RevoluteJointBuilder::new(Vec3::Y)
            //.local_anchor1(Vec3::new(0.0, 0.0, 0.0))
            .local_anchor2(Vec3::new(0.0, -height - Self::hy(), 0.0))
            .motor_model(MotorModel::AccelerationBased)
            .motor_velocity(10.0, 1.0)
            .into()
    }
}

fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_ground: Query<Entity, With<Ground>>,
) {
    if let Ok(ground_entity) = q_ground.get_single() {
        commands
            .spawn((
                Name::new("PADDLE"),
                Paddle,
                // PbrBundle {
                //     mesh: meshes.add(Paddle::shape().into()),
                //     transform: Transform::from_xyz(0.0, Paddle::hy() + SPACE, 0.0),
                //     //material: materials.add(Color::BLUE.into()),
                //     ..default()
                // },
                Transform::from_xyz(0.0, Paddle::hy() + SPACE, 0.0),
            ))
            .insert((
                RigidBody::Dynamic,
                Sleeping::disabled(),
                Paddle::collider(),
                ImpulseJoint::new(ground_entity, Paddle::joint(0.2)),
            ));
    }
}

fn move_paddle(mut q_paddles: Query<&mut ImpulseJoint, With<Paddle>>, keys: Res<Input<KeyCode>>) {
    let left = keys.pressed(KeyCode::Left);
    let right = keys.pressed(KeyCode::Right);
    let (velocity, factor) = if left {
        (30.0, 1.0)
    } else if right {
        (-30.0, 10.0)
    } else {
        (0.0, 10.0)
    };
    for mut joint in q_paddles.iter_mut() {
        if let Some(joint) = joint.data.as_revolute_mut() {
            // info!("move_paddle set_motor_velocity({velocity}, {factor})");
            joint.set_motor_velocity(velocity, factor);
        }
    }
}

fn set_motor_pos(mut q_paddles: Query<&mut ImpulseJoint, With<Ground>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        info!("set_motor_position");
        let pos = 90.0;
        let stiffness = 1.0;
        let damping = 1.0;
        for mut joint in q_paddles.iter_mut() {
            if let Some(joint) = joint.data.as_revolute_mut() {
                joint.set_motor_position(pos, stiffness, damping);
            }
        }
    }
}

fn debug(
    q_paddles: Query<&Transform, With<Paddle>>,
    q_joint: Query<&ImpulseJoint>,
    q_collider: Query<&Collider, With<Paddle>>,
) {
    if let Ok(paddle_transform) = q_paddles.get_single() {
        info!("DEBUG paddle pos: {paddle_transform:?}");
    }
    if let Ok(joint) = q_joint.get_single() {
        info!("DEBUG joint pos: {joint:?}");
    }
    if let Ok(collider) = q_collider.get_single() {
        info!("DEBUG joint pos: {collider:?}");
    }
}
