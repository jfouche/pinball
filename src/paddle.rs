use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::Ground;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_paddle)
            .add_systems(Update, (move_paddle,));
    }
}

#[derive(Component)]
pub struct Paddle;

const PADDLE_SIZE: Vec3 = Vec3::new(5.0, 0.5, 2.0);

fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_ground: Query<Entity, With<Ground>>,
) {
    // create_revolute_joints(&mut commands, Vec3::new(20.0, 0.0, 0.0), 3);

    if let Ok(ground_entity) = q_ground.get_single() {
        let space = 0.3;
        let joint = RevoluteJointBuilder::new(Vec3::Y)
            .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
            .local_anchor2(Vec3::new(0.0, -space, 0.0))
            .motor_model(MotorModel::AccelerationBased)
            .motor_velocity(0.0, 1.0);

        commands
            .spawn((
                Name::new("PADDLE"),
                Paddle,
                PbrBundle {
                    mesh: meshes
                        .add(shape::Box::new(PADDLE_SIZE.x, PADDLE_SIZE.y, PADDLE_SIZE.z).into()),
                    //material: materials.add(Color::BLUE.into()),
                    transform: Transform::from_xyz(0.0, PADDLE_SIZE.y / 2.0 + space, 0.0),
                    ..default()
                },
            ))
            .insert((
                RigidBody::Dynamic,
                Collider::cuboid(
                    PADDLE_SIZE.x / 2.0,
                    PADDLE_SIZE.y / 2.0,
                    PADDLE_SIZE.z / 2.0,
                ),
                ImpulseJoint::new(ground_entity, joint),
            ));
    }
}

fn move_paddle(mut q_paddles: Query<&mut ImpulseJoint, With<Paddle>>, keys: Res<Input<KeyCode>>) {
    let left = keys.pressed(KeyCode::Left);
    for mut joint in q_paddles.iter_mut() {
        let (velocity, factor) = if left { (30.0, 1.0) } else { (0.0, 1.0) };
        if let Some(joint) = joint.data.as_revolute_mut() {
            info!("move_paddle {velocity}, {factor}");
            joint.set_motor_velocity(velocity, factor);
        }
    }
}

fn debug(q_paddles: Query<&ImpulseJoint, With<Paddle>>) {
    for joint in q_paddles.iter() {
        if let Some(joint) = joint.data.as_revolute() {
            info!("debug {joint:?}");
        }
    }
}

fn create_revolute_joints(commands: &mut Commands, origin: Vec3, num: usize) {
    let rad = 0.4;
    let shift = 2.0;

    let mut curr_parent = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(origin.x, origin.y, 0.0)),
            RigidBody::Fixed,
            Collider::cuboid(rad, rad, rad),
        ))
        .id();

    for i in 0..num {
        // Create four bodies.
        let z = origin.z + i as f32 * shift * 2.0 + shift;
        let positions = [
            Vec3::new(origin.x, origin.y, z),
            Vec3::new(origin.x + shift, origin.y, z),
            Vec3::new(origin.x + shift, origin.y, z + shift),
            Vec3::new(origin.x, origin.y, z + shift),
        ];

        let mut handles = [curr_parent; 4];
        for k in 0..4 {
            handles[k] = commands
                .spawn((
                    TransformBundle::from(Transform::from_translation(positions[k])),
                    RigidBody::Dynamic,
                    Collider::cuboid(rad, rad, rad),
                ))
                .id();
        }

        // Setup four joints.
        let x = Vec3::X;
        let z = Vec3::Z;

        let revs = [
            //            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, 0.0)),
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(-shift, 0.0, 0.0)),
            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(shift, 0.0, 0.0)),
        ];

        commands
            .entity(handles[0])
            .insert(ImpulseJoint::new(curr_parent, revs[0]));
        commands
            .entity(handles[1])
            .insert(ImpulseJoint::new(handles[0], revs[1]));
        commands
            .entity(handles[2])
            .insert(ImpulseJoint::new(handles[1], revs[2]));
        commands
            .entity(handles[3])
            .insert(ImpulseJoint::new(handles[2], revs[3]));

        curr_parent = handles[3];
    }
}
