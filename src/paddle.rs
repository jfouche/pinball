use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_paddle, set_motor_pos));
    }
}

#[derive(Clone, Component)]
pub struct Paddle;

impl Paddle {
    const SIZE: Vec3 = Vec3::new(5.0, 0.5, 2.0);
    /// space between Paddle and board
    const SPACE: f32 = 2.5;

    fn hx() -> f32 {
        Self::SIZE.x / 2.0
    }

    fn hy() -> f32 {
        Self::SIZE.y / 2.0
    }

    fn hz() -> f32 {
        Self::SIZE.z / 2.0
    }

    fn dx() -> f32 {
        // -4.0 * Self::hx() / 5.0
        -Self::hx()
    }

    #[allow(dead_code, unused_variables)]
    fn shape() -> impl Into<Mesh> {
        shape::Box::new(Self::SIZE.x, Self::SIZE.y, Self::SIZE.z)
    }

    fn collider() -> Collider {
        Collider::cuboid(Self::hx(), Self::hy(), Self::hz())
    }

    fn transform(pos: &Vec3) -> Transform {
        Transform::from_xyz(pos.x + Self::dx(), Self::hy() + Self::SPACE, pos.z)
    }

    fn joint(pos: &Vec3) -> impl Into<GenericJoint> {
        // parent entity anchor position
        let xp = pos.x;
        let yp = 0.0;
        let zp = pos.z;
        let parent_pos = Vec3::new(xp, yp, zp);
        // paddle anchor position
        let x = -Self::hx(); // + Self::dx();
        let y = -Self::SPACE - Self::hy();
        let z = 0.0;
        let paddle_pos = Vec3::new(x, y, z);
        info!("Paddle::joint({pos}) : parent={parent_pos} paddle={paddle_pos}");
        RevoluteJointBuilder::new(Vec3::Y)
            .local_anchor1(parent_pos)
            .local_anchor2(paddle_pos)
            .motor_model(MotorModel::AccelerationBased)
            .motor_velocity(0.0, 1.0)
    }
}

pub fn spawn_paddle(builder: &mut ChildBuilder, pos: Vec3) {
    let board_entity = builder.parent_entity();
    builder
        .spawn((
            Name::new("PADDLE"),
            Paddle,
            // PbrBundle {
            //     mesh: meshes.add(Paddle::shape().into()),
            //     transform: Transform::from_xyz(0.0, Paddle::hy() + SPACE, 0.0),
            //     //material: materials.add(Color::BLUE.into()),
            //     ..default()
            // },
            Paddle::transform(&pos),
        ))
        .insert((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            Paddle::collider(),
            ImpulseJoint::new(board_entity, Paddle::joint(&pos)),
        ));
}

fn move_paddle(mut q_paddles: Query<&mut ImpulseJoint, With<Paddle>>, keys: Res<Input<KeyCode>>) {
    let left = keys.pressed(KeyCode::Left);
    let right = keys.pressed(KeyCode::Right);
    if left || right {
        let (velocity, factor) = if left { (30.0, 1.0) } else { (-30.0, 10.0) };
        for mut joint in q_paddles.iter_mut() {
            if let Some(joint) = joint.data.as_revolute_mut() {
                // info!("move_paddle set_motor_velocity({velocity}, {factor})");
                joint.set_motor_velocity(velocity, factor);
            }
        }
    }
}

fn set_motor_pos(mut q_paddles: Query<&mut ImpulseJoint, With<Paddle>>, keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Space) {
        let target_pos = 1.5;
        let target_vel = 80.0;
        let stiffness = 30.0;
        let damping = 2000.0;
        for mut impulse_joint in q_paddles.iter_mut() {
            if let Some(joint) = impulse_joint.data.as_revolute_mut() {
                info!("set_motor({target_pos}, {target_vel}, {stiffness}, {damping})");
                joint.set_motor_position(target_pos, stiffness, damping);
                // joint.set_motor_velocity(target_vel, 100.0);
            }
        }
    }
}
