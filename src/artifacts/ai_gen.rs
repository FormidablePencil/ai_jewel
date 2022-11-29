use crate::artifacts::data_center::DataCenter;
use crate::artifacts::desire_variables::DesireVariables;
use crate::artifacts::game::{Game, Player};
use crate::artifacts::operations::OperationsCenter;
use std::collections::HashMap;

pub struct MyTask {
    pub date: String,
    pub instructions: String,
}

fn ai_gen() {
    Ai {
        // database: &data_center,
        operations_center: OperationsCenter {
            database: &mut DataCenter::default_data_center(),
            game: Game::new(),
        },
    };
}

pub struct Ai<'a> {
    // database: &'a DataCenter,
    operations_center: OperationsCenter<'a>,
}

pub enum RequestAi {
    PlayGame,      // playing ticktacktoe game
    Tasks,         // tasks game
    DiscernDesire, // which is easier game
}

/*
   Creates an ai upon instantiation
*/
impl<'a> Ai<'a> {
    fn new_task(&mut self, task: MyTask) -> bool {
        // TaskExample {date: String::from("")}
        // self.database.insert_task(task, MyTask { date: "".to_string(), instructions: "".to_string() })
        todo!()
    }

    fn generic_request_ai(&mut self, request_ai: RequestAi) {
        // data gets to choose
        match request_ai {
            RequestAi::PlayGame => {}
            RequestAi::Tasks => {}
            RequestAi::DiscernDesire => {}
        }

        // pub fn start_game(&self) {
        let player_starts = Player::One;
        self.operations_center.game.start_game(player_starts)
        // }
    }

    fn request_desired_generic_requests() {
        // eventually depending on the timing the desire will change
    }

    fn request_ai(&mut self, request: u128) {
        // for this ai will have to generate options for me...
    }
}
