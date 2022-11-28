use std::collections::HashMap;
use crate::artifacts::ai_gen::hello::Ai;
use crate::artifacts::data_center::DataCenter;
use crate::artifacts::desire_variables::DesireVariables;
use crate::artifacts::game::{Game, Player};
use crate::artifacts::operations::OperationsCenter;

pub struct MyTask {
    pub date: String,
    pub instructions: String,
}

fn ai_gen() {
    Ai::new();
}

pub type MyDatabase = DataCenter<MyTask, DesireVariables>;

pub struct Ai<'a, Task, DesireVariables> {
    database: &'a MyDatabase,
    operations_center: OperationsCenter<'a, Task, DesireVariables>,
}

pub enum RequestAi {
    PlayGame, // playing ticktacktoe game
    Tasks, // tasks game
    DiscernDesire, // which is easier game
}

/*
    Creates an ai upon instantiation
 */
impl Ai<MyTask, DesireVariables> {
    fn new() -> Self {
        let database = DataCenter {
            unique_index: 0,
            tasks: HashMap::new() as HashMap<u128, MyTask>,
            desire_variables: HashMap::new() as HashMap<u128, DesireVariables>,
            alive: true,
        };
        Self {
            database: &database,
            operations_center: OperationsCenter{
                database: &database,
                game: Game::new()
            },
        }
    }

    fn new_task(&mut self, task: MyTask) -> bool {

        // TaskExample {date: String::from("")}
        self.database.insert_task(task)
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
