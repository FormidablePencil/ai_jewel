use core::panicking::panic;
use std::collections::HashMap;
use crate::artifacts::attachments::FuckedUp;

pub enum Player {
    One,
    Two,
}

pub struct Game<'a> {
    pub fucked_up: FuckedUp,
    pub started: bool,
    pub player_turn: Player,
    pub score: &'a HashMap<u8, Player>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            fucked_up: FuckedUp::new(),
            started: false,
            player_turn: Player::One,
            score: Default::default(),
        }
    }

    pub fn start_game(&mut self, player_starts: Player) {
        self.started = true;
        self.player_turn = player_starts;
    }

    fn choose(&mut self, square_index: u8) {
        if self.score.contains_key(&square_index) {
            // todo - common sense achieved. consciousness achieved once error is not reliant anymore.
            // todo - counter how many times this mistake was made.
            self.fucked_up(String::from("Square already taken. Assumed to be internal error. To be handled by assumption."));
        } else {
            self.score.insert(square_index, *self.player_turn)
        }
    }
}
