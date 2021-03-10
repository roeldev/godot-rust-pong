use crate::ball;
use gdnative::prelude::*;
use gdnative::api::{Node, KinematicBody2D, Label};
use std::ops::Deref;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Level {
    scores: (u32, u32),
    player_score: Ref<Label>,
    opponent_score: Ref<Label>,
    ball: Ref<KinematicBody2D>,
}

#[methods]
impl Level {
    fn new(_owner: &Node) -> Self {
        Level {
            scores: (0, 0),
            player_score: Label::new().into_shared(),
            opponent_score: Label::new().into_shared(),
            ball: KinematicBody2D::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        self.player_score = find_node::<Label>(owner, "PlayerScore");
        self.opponent_score = find_node::<Label>(owner, "OpponentScore");
        self.ball = find_node::<KinematicBody2D>(owner, "Ball");

        update_score_label(self.player_score, 0);
        update_score_label(self.opponent_score, 0);
    }

    // todo: areas splitsen. ball pos check is niet accuraat
    #[export]
    fn _on_side_area_body_entered(&mut self, _owner: &Node, _body: Ref<Node>) {
        let ball_position = unsafe { self.ball.assume_safe().position() };
        if ball_position.x < 0.0 {
            self.scores.1 += 1;
            update_score_label(self.opponent_score, self.scores.1);
        } else {
            self.scores.0 += 1;
            update_score_label(self.player_score, self.scores.0);
        }

        // reset ball
        self.get_ball_instance()
            .map_mut(|ball, _| {
                ball.reset_position();
                ball.reset_velocity();
            })
            .expect("Failed to reset Ball");
    }

    pub fn get_ball_owner(&self) -> Ref<KinematicBody2D> { self.ball }

    pub fn get_ball_instance(&self) -> RefInstance<ball::Ball, Shared> {
        let ball = unsafe { self.ball.assume_safe() };
        ball.cast_instance::<ball::Ball>()
            .expect("Failed to cast ball to instance")
    }
}

fn find_node<T>(owner: &Node, name: &str) -> Ref<T, Shared>
    where T: GodotObject
{
    owner
        .find_node(GodotString::from_str(name), false, false)
        .expect(format!("Failed to find `{}` node", name).deref())
        .to_variant()
        .try_to_object::<T>()
        .expect(format!("Failed to cast `{}` node", name).deref())
}

fn update_score_label(label: Ref<Label>, score: u32) {
    let label = unsafe { label.assume_safe() };
    label.set_text(score.to_string());
}

