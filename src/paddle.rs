use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_paddle,));
    }
}

#[derive(Clone, Component)]
pub struct Paddle {
    pos: Vec3,
}

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

    fn shape() -> impl Into<Mesh> {
        shape::Box::new(Self::SIZE.x, Self::SIZE.y, Self::SIZE.z)
    }

    fn collider() -> Collider {
        Collider::cuboid(Self::hx(), Self::hy(), Self::hz())
    }

    fn transform(&self) -> Transform {
        Transform::from_xyz(self.pos.x, Self::hy() + Self::SPACE, self.pos.z)
    }

    fn joint(&self) -> impl Into<GenericJoint> {
        let dx = -4.0 * Self::hx() / 5.0;
        let x = self.pos.x;
        let z = self.pos.z;
        RevoluteJointBuilder::new(Vec3::Y)
            .local_anchor1(Vec3::new(x, 0.0, z))
            .local_anchor2(Vec3::new(x - dx, -Self::SPACE - Self::hy(), z))
            .motor_model(MotorModel::AccelerationBased)
            .motor_velocity(0.0, 1.0)
    }
}

pub fn spawn_paddle(builder: &mut ChildBuilder, pos: Vec3) {
    let paddle = Paddle { pos };
    let transform = paddle.transform();
    let impulse_joint = ImpulseJoint::new(builder.parent_entity(), paddle.joint());
    builder
        .spawn((
            Name::new("PADDLE"),
            paddle,
            // PbrBundle {
            //     mesh: meshes.add(Paddle::shape().into()),
            //     transform: Transform::from_xyz(0.0, Paddle::hy() + SPACE, 0.0),
            //     //material: materials.add(Color::BLUE.into()),
            //     ..default()
            // },
            transform,
        ))
        .insert((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            Paddle::collider(),
            impulse_joint,
        ));
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

// #[allow(dead_code)]
// fn set_motor_pos(mut q_paddles: Query<&mut ImpulseJoint, With<Ground>>, keys: Res<Input<KeyCode>>) {
//     if keys.just_pressed(KeyCode::Space) {
//         info!("set_motor_position");
//         let pos = 90.0;
//         let stiffness = 1.0;
//         let damping = 1.0;
//         for mut joint in q_paddles.iter_mut() {
//             if let Some(joint) = joint.data.as_revolute_mut() {
//                 joint.set_motor_position(pos, stiffness, damping);
//             }
//         }
//     }
// }
