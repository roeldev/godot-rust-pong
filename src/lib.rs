use gdnative::prelude::*;

extern crate rand;

mod level;
mod ball;
mod opponent;
mod player;
mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<level::Level>();
    handle.add_class::<ball::Ball>();
    handle.add_class::<opponent::Opponent>();
    handle.add_class::<player::Player>();
}

godot_init!(init);
