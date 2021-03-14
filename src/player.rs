// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;
use gdnative::api::{KinematicBody2D};
use std::ops::Mul;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    #[property(default = 400.0)]
    speed: f32,
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            speed: 400.0,
        }
    }

    // #[export]
    // fn _ready(&self, _owner: &KinematicBody2D) {
    //     godot_print!("player ready");
    // }

    #[export]
    fn _process(&self, owner: &KinematicBody2D, _dt: f32) {
        let mut velocity = Vector2::zero();

        let input = Input::godot_singleton();
        if input.is_action_pressed("ui_up") {
            velocity.y -= 1.0;
        }
        if input.is_action_pressed("ui_down") {
            velocity.y += 1.0;
        }

        owner.move_and_slide(
            velocity.mul(self.speed),
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
    }
}
