// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;
use gdnative::api::{KinematicBody2D};
use std::ops::{Mul};

use crate::level;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Opponent {
    #[property(default = 400.0)]
    speed: f32,
    #[property(default = 10.0)]
    accuracy: f32,

    level: Ref<Node>,
}

#[methods]
impl Opponent {
    fn new(_owner: &KinematicBody2D) -> Self {
        Opponent {
            speed: 400.0,
            accuracy: 10.0,
            level: Node::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        // todo: figure out how to do with trait
        self.level = owner.
            find_parent(GodotString::from_str("Level"))
            .expect("Should find Level")
    }

    #[export]
    fn _process(&self, owner: &KinematicBody2D, _dt: f32) {
        owner.move_and_slide(
            self.get_direction(owner.position()).mul(self.speed),
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
    }

    fn get_level_instance(&self) -> RefInstance<level::Level, Shared> {
        let level = unsafe { self.level.assume_safe() };
        level.cast_instance::<level::Level>()
            .expect("Failed to cast level to instance")
    }

    pub fn get_direction(&self, position: Vector2) -> Vector2 {
        let ball = self.get_level_instance()
            .map(|level, _| { level.get_ball_owner() })
            .expect("Failed to get ball from Level");

        let ball = unsafe { ball.assume_safe() };
        let diff = ball.position().y - position.y;

        return Vector2::new(
            0.0,
            if diff > self.accuracy { 1.0 } else if diff < -self.accuracy { -1.0 } else { 0.0 },
        );
    }
}
