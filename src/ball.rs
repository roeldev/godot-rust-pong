use gdnative::prelude::*;
use gdnative::api::{KinematicBody2D, OS};
use std::ops::Mul;
use rand::Rng;
use std::borrow::BorrowMut;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Ball {
    #[property(default = 600.0)]
    pub speed: f32,

    pub velocity: Vector2,
    owner: Ref<KinematicBody2D>,
}

#[methods]
impl Ball {
    fn new(owner: &KinematicBody2D) -> Self {
        Ball {
            speed: 600.0,
            velocity: rand_velocity(),
            owner: unsafe { owner.assume_shared() },
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody2D) {
        self.reset_position();
        self.borrow_mut().reset_velocity();
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, dt: f32) {
        match owner.move_and_collide(self.velocity.mul(self.speed * dt), true, true, false) {
            Some(collision) => {
                let collision = unsafe { collision.assume_safe() };
                self.velocity = self.velocity.reflect(collision.normal());
            }
            _ => {}
        }
    }

    pub fn reset_position(&self) {
        let owner = unsafe { self.owner.assume_safe() };
        owner.set_position(OS::godot_singleton().window_size().mul(0.5));
    }

    pub fn reset_velocity(&mut self) { self.velocity = rand_velocity(); }
}

fn rand_velocity() -> Vector2 {
    let mut rng = rand::thread_rng();
    Vector2::new(
        if rng.gen_range(-1..1) < 0 { -1.0 } else { 1.0 },
        if rng.gen_range(-1..1) < 0 { -0.8 } else { 0.8 },
    )
}