// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::api::{AudioStreamPlayer, KinematicBody2D, Label, Node};
use gdnative::prelude::*;

use crate::ball;
use crate::utils::node::NodeRef;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Level {
    scores: (u32, u32),
    score_sound: NodeRef<AudioStreamPlayer>,
    player_score: NodeRef<Label>,
    opponent_score: NodeRef<Label>,
    ball: NodeRef<KinematicBody2D>,
    countdown_timer: NodeRef<Timer>,
    countdown_label: NodeRef<Label>,
}

#[methods]
impl Level {
    fn new(_owner: &Node) -> Self {
        Level {
            scores: (0, 0),
            score_sound: NodeRef::new("ScoreSound"),
            player_score: NodeRef::new("PlayerScore"),
            opponent_score: NodeRef::new("OpponentScore"),
            ball: NodeRef::new("Ball"),
            countdown_timer: NodeRef::new("CountdownTimer"),
            countdown_label: NodeRef::new("CountdownLabel"),
        }
    }

    #[inline]
    pub fn get_ball_owner(&self) -> TRef<KinematicBody2D> { self.ball.get_ref() }

    #[allow(dead_code)]
    #[inline]
    pub fn get_ball_instance(&self) -> RefInstance<ball::Ball, Shared> {
        self.get_ball_owner()
            .cast_instance::<ball::Ball>()
            .expect("Failed to cast ball to instance")
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        self.score_sound.get_from(owner);
        self.player_score.get_from(owner);
        self.opponent_score.get_from(owner);
        self.ball.get_from(owner);
        self.countdown_timer.get_from(owner);
        self.countdown_label.get_from(owner);

        update_score_label(self.player_score.get_ref(), 0);
        update_score_label(self.opponent_score.get_ref(), 0);
    }

    #[export]
    fn _process(&self, _owner: &Node, _dt: f32) {
        let label = self.countdown_label.get_ref();
        if label.is_visible() {
            let timer = self.countdown_timer.get_ref();
            let time_left = timer.time_left().ceil() as u32;

            if time_left > 0 {
                label.set_text(time_left.to_string());
            } else {
                label.set_visible(false);
            }
        }
    }

    // Opponent scores
    #[export]
    fn _on_left_area_body_entered(&mut self, owner: &Node, _body: Ref<Node>) {
        self.scores.1 += 1;
        update_score_label(self.opponent_score.get_ref(), self.scores.1);
        self.score_achieved(owner, true);
    }

    // Player scores
    #[export]
    fn _on_right_area_body_entered(&mut self, owner: &Node, _body: Ref<Node>) {
        self.scores.0 += 1;
        update_score_label(self.player_score.get_ref(), self.scores.0);
        self.score_achieved(owner, true);
    }

    #[export]
    fn _on_countdown_timer_timeout(&self, owner: &Node) {
        owner.get_tree()
            .map(|tree| unsafe { tree.assume_safe() })
            .map(|tree| { tree.call_group("BallGroup", "restart", &[]); })
            .expect("Failed to get SceneTree");
    }

    fn score_achieved(&self, owner: &Node, countdown: bool) {
        let tree = unsafe {
            owner.get_tree()
                .expect("Failed to get SceneTree")
                .assume_safe()
        };

        let score_sound = self.score_sound.get_ref();
        score_sound.play(0.0);

        if !countdown {
            tree.call_group("BallGroup", "reset", &[]);
            return;
        }

        tree.call_group("BallGroup", "reset_and_stop", &[]);

        // make countdown label visible
        let label = self.countdown_label.get_ref();
        label.set_visible(true);
        label.set_text("3");

        // start counting down
        let timer = self.countdown_timer.get_ref();
        timer.start(2.5);
    }
}

#[inline]
fn update_score_label(label: TRef<Label>, score: u32) {
    label.set_text(score.to_string());
}