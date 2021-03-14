use crate::{ball, utils::find_node};
use gdnative::prelude::*;
use gdnative::api::{Node, KinematicBody2D, Label, AudioStreamPlayer};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Level {
    scores: (u32, u32),
    score_sound: Ref<AudioStreamPlayer>,
    player_score: Ref<Label>,
    opponent_score: Ref<Label>,
    ball: Ref<KinematicBody2D>,
    countdown_timer: Ref<Timer>,
    countdown_label: Ref<Label>,
}

#[methods]
impl Level {
    fn new(_owner: &Node) -> Self {
        Level {
            scores: (0, 0),
            score_sound: AudioStreamPlayer::new().into_shared(),
            player_score: Label::new().into_shared(),
            opponent_score: Label::new().into_shared(),
            ball: KinematicBody2D::new().into_shared(),
            countdown_timer: Timer::new().into_shared(),
            countdown_label: Label::new().into_shared(),
        }
    }

    pub fn get_ball_owner(&self) -> Ref<KinematicBody2D> { self.ball }

    #[allow(dead_code)]
    pub fn get_ball_instance(&self) -> RefInstance<ball::Ball, Shared> {
        let ball = unsafe { self.ball.assume_safe() };
        ball.cast_instance::<ball::Ball>()
            .expect("Failed to cast ball to instance")
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        self.score_sound = find_node::<AudioStreamPlayer>(owner, "ScoreSound");
        self.player_score = find_node::<Label>(owner, "PlayerScore");
        self.opponent_score = find_node::<Label>(owner, "OpponentScore");
        self.ball = find_node::<KinematicBody2D>(owner, "Ball");
        self.countdown_timer = find_node::<Timer>(owner, "CountdownTimer");
        self.countdown_label = find_node::<Label>(owner, "CountdownLabel");

        update_score_label(self.player_score, 0);
        update_score_label(self.opponent_score, 0);
    }

    #[export]
    fn _process(&self, _owner: &Node, _dt: f32) {
        let label = unsafe { self.countdown_label.assume_safe() };
        if label.is_visible() {
            let timer = unsafe { self.countdown_timer.assume_safe() };
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
        update_score_label(self.opponent_score, self.scores.1);
        self.score_achieved(owner, true);
    }

    // Player scores
    #[export]
    fn _on_right_area_body_entered(&mut self, owner: &Node, _body: Ref<Node>) {
        self.scores.0 += 1;
        update_score_label(self.player_score, self.scores.0);
        self.score_achieved(owner, true);
    }

    #[export]
    fn _on_countdown_timer_timeout(&self, owner: &Node) {
        let tree = unsafe {
            owner.get_tree()
                .expect("Failed to get SceneTree")
                .assume_safe()
        };

        tree.call_group(
            GodotString::from_str("BallGroup"),
            GodotString::from_str("restart"),
            &[],
        );
    }

    fn score_achieved(&self, owner: &Node, countdown: bool) {
        let tree = unsafe {
            owner.get_tree()
                .expect("Failed to get SceneTree")
                .assume_safe()
        };

        let score_sound = unsafe { self.score_sound.assume_safe() };
        score_sound.play(0.0);

        if countdown {
            tree.call_group(
                GodotString::from_str("BallGroup"),
                GodotString::from_str("reset_and_stop"),
                &[],
            );

            // make countdown label visible
            let label = unsafe { self.countdown_label.assume_safe() };
            label.set_visible(true);
            label.set_text(GodotString::from_str("3"));

            // start counting down
            let timer = unsafe { self.countdown_timer.assume_safe() };
            timer.start(2.5);
        } else {
            tree.call_group(
                GodotString::from_str("BallGroup"),
                GodotString::from_str("reset"),
                &[],
            );
        }

        // self.get_ball_instance()
        //     .map_mut(|ball, _| {
        //         ball.reset();
        //     })
        //     .expect("Failed to reset Ball");
    }
}

fn update_score_label(label: Ref<Label>, score: u32) {
    let label = unsafe { label.assume_safe() };
    label.set_text(score.to_string());
}