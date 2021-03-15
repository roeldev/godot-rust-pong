// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::{utils::find_node};
use gdnative::prelude::*;
use gdnative::api::{KinematicBody2D, OS, AudioStreamPlayer};
use std::ops::Mul;
use rand::Rng;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Ball {
    #[property(default = 600.0)]
    pub speed: f32,

    velocity: Vector2,
    collision_sound: Ref<AudioStreamPlayer>,
}

#[methods]
impl Ball {
    fn new(_owner: &KinematicBody2D) -> Self {
        Ball {
            speed: 600.0,
            velocity: rand_velocity(),
            collision_sound: AudioStreamPlayer::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        reset_position(owner);
        self.collision_sound = find_node::<AudioStreamPlayer>(owner, "CollisionSound");
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, dt: f32) {
        match owner.move_and_collide(self.velocity.mul(self.speed * dt), true, true, false) {
            Some(collision) => {
                let sound = unsafe { self.collision_sound.assume_safe() };
                sound.play(0.0);

                let collision = unsafe { collision.assume_safe() };
                self.velocity = self.velocity.reflect(collision.normal());
                self.speed *= 1.01
            }
            _ => {}
        }
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

fn reset_position(owner: &KinematicBody2D) {
    owner.set_position(OS::godot_singleton().window_size().mul(0.5));
}

fn rand_velocity() -> Vector2 {
    let mut rng = rand::thread_rng();
    Vector2::new(
        if rng.gen_range(-1..1) < 0 { -1.0 } else { 1.0 },
        if rng.gen_range(-1..1) < 0 { -0.8 } else { 0.8 },
    )
}