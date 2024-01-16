use bevy::prelude::*;
use bevy_rapier3d::dynamics::{GenericJoint, RevoluteJointBuilder};
use serde::{Deserialize, Serialize};

use crate::paddle::{Paddle, PaddleType};

#[derive(Debug, Deserialize, Serialize)]
pub struct PinballConfig {
    pub board: BoardConfig,
    pub paddles: Vec<PaddleConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardConfig {
    pub size: Vec3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaddleConfig {
    pub pos: Vec3,
    pub ptype: PaddleType,
}

impl PaddleConfig {
    fn x_offset(&self) -> f32 {
        match self.ptype {
            PaddleType::Left => -Paddle::hx(),
            PaddleType::Right => Paddle::hx(),
        }
    }

    pub fn transform(&self) -> Transform {
        let x = self.pos.x - self.x_offset();
        let y = Paddle::hy() + Paddle::SPACE;
        let z = self.pos.z;
        Transform::from_xyz(x, y, z)
    }

    /// Create an joint object, based on :
    /// - anchor1 (parent position): position on the ground (the `pos` param)
    /// - anchor2 (paddle position): position of the point of rotation
    ///
    pub fn joint(&self) -> impl Into<GenericJoint> {
        // parent entity anchor position is self.pos, forced y at 0
        let parent_pos = Vec3::new(self.pos.x, 0.0, self.pos.z);
        // paddle anchor position is at x border, at [SPACE] height
        let paddle_pos = Vec3::new(self.x_offset(), -Paddle::SPACE - Paddle::hy(), 0.0);
        info!("Joint : parent_pos = {parent_pos}, paddle_pos = {paddle_pos}");
        RevoluteJointBuilder::new(Vec3::Y)
            .local_anchor1(parent_pos)
            .local_anchor2(paddle_pos)
            .limits([-0.4, 0.4])
    }
}

pub fn load_config() -> PinballConfig {
    let content = include_str!("../assets/level.json");
    let config = serde_json::from_str(content).expect("can't deserialize level");
    info!("Load config : {config:?}");
    config
}

pub fn test() {
    let config = PinballConfig {
        board: BoardConfig {
            size: Vec3::new(20.0, 0.0, 20.0),
        },
        paddles: vec![
            PaddleConfig {
                pos: Vec3::new(12.0, 0.0, 18.0),
                ptype: PaddleType::Right,
            },
            PaddleConfig {
                pos: Vec3::new(8.0, 0.0, 18.0),
                ptype: PaddleType::Left,
            },
        ],
    };
    let s = serde_json::to_string(&config).unwrap();
    info!("Config test : {s}");
}
