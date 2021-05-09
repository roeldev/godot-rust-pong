// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::ops::Mul;

use gdnative::api::{AudioStreamPlayer, KinematicBody2D, OS};
use gdnative::prelude::*;
use rand::Rng;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Ball {
    #[property(default = 600.0)]
    pub speed: f32,

    velocity: Vector2,
}

#[methods]
impl Ball {
    fn new(_owner: &KinematicBody2D) -> Self {
        Ball {
            speed: 600.0,
            velocity: rand_velocity(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        reset_position(owner);
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, dt: f32) {
        let collision = owner.move_and_collide(self.velocity.mul(self.speed * dt), true, true, false);
        if collision.is_none() {
            return;
        }

        if let Some(sound) = unsafe { owner.get_node_as::<AudioStreamPlayer>("CollisionSound") } {
            sound.play(0.0)
        }

        let collision = collision.unwrap();
        let collision = unsafe { collision.assume_safe() };
        self.velocity = self.velocity.reflect(collision.normal());
        self.speed *= 1.01
    }

    #[export]
    fn reset(&mut self, owner: &KinematicBody2D) {
        reset_position(owner);
        self.velocity = rand_velocity();
    }

    #[export]
    fn reset_and_stop(&mut self, owner: &KinematicBody2D) {
        self.speed = 0.0;
        self.reset(owner);
    }

    #[export]
    fn restart(&mut self, _owner: &KinematicBody2D) {
        self.speed = 600.0;
    }
}

#[inline]
fn reset_position(owner: &KinematicBody2D) {
    owner.set_position(OS::godot_singleton().window_size().mul(0.5));
}

#[inline]
fn rand_velocity() -> Vector2 {
    let mut rng = rand::thread_rng();
    Vector2::new(
        if rng.gen_range(-1..1) < 0 { -1.0 } else { 1.0 },
        if rng.gen_range(-1..1) < 0 { -0.8 } else { 0.8 },
    )
}