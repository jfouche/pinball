use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::Ground;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_paddle);
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
    if let Ok(ground_entity) = q_ground.get_single() {
        let joint = RevoluteJointBuilder::new(Vec3::Y)
            .local_anchor1(Vec3::new(0.0, PADDLE_SIZE.y / 2.0, 0.0))
            .local_anchor2(Vec3::new(2.0, 0.0, 0.0))
            .motor_velocity(1.0, 1.0);
        commands
            .spawn((
                Name::new("PADDLE"),
                Paddle,
                PbrBundle {
                    mesh: meshes
                        .add(shape::Box::new(PADDLE_SIZE.x, PADDLE_SIZE.y, PADDLE_SIZE.z).into()),
                    material: materials.add(Color::BLUE.into()),
                    transform: Transform::from_xyz(2.0, PADDLE_SIZE.y / 2.0, 2.0),
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
                Velocity::default(),
                ExternalForce::default(),
                Damping {
                    linear_damping: 0.5,
                    ..Default::default()
                },
                ImpulseJoint::new(ground_entity, joint),
            ));
    }
}

// fn despawn_ball(mut commands: Commands, player_query: Query<Entity, With<Ball>>) {
//     if let Ok(player_entity) = player_query.get_single() {
//         commands.entity(player_entity).despawn();
//     }
// }
