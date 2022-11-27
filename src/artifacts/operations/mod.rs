use crate::artifacts::database::Database;
use crate::artifacts::game::Game;

mod tasks;

pub struct OperationsCenter<'a, Task, DesireVariables> {
    pub database: &'a Database<Task, DesireVariables>,
    pub game: Game<'a>,
}

pub struct ComparedRelationship {
    pub winner_id: u128,
    pub looser_id: u128,
    abstraction_level: u128,
    // if 0 look id of tasks and not winning relationships
    reason: String,
}

pub struct NewRelationship {
    pub first_id: u128,
    pub first_abstract_lvl: u128,
    pub second_id: u128,
    pub second_abstract_lvl: u128,
    pub winner_is_first: bool,
    pub reason: String,
}

impl<TaskExample, DesireVariables> OperationsCenter<TaskExample, DesireVariables> {
    pub fn create_relationship(&mut self, nr: NewRelationship) {
        if nr.first_abstract_lvl == 0 || nr.second_abstract_lvl == 0 {
            self.database.compare_with_abstract_level_0_task(nr);
        } else {
            self.database.new_relationship(nr)
        }
    }

    // pub fn start_game(&self) {
    //     self.start_game()
    // }
}
