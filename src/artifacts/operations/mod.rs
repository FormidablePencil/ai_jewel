use crate::artifacts::data_center::DataCenter;
use crate::artifacts::game::Game;

mod task_operators;
mod tasks;

pub struct ComparedRelationship {
    pub winner_id: u128,
    pub looser_id: u128,
    abstraction_level: u128,
    // if 0 look id of tasks and not winning relationships
    reason: String,
}

pub struct NewTaskRelationship {
    pub first_id: u128,
    pub first_abstract_lvl: u128,
    pub second_id: u128,
    pub second_abstract_lvl: u128,
    pub winner_is_first: bool,
    pub reason: String,
}

pub struct OperationsCenter<'a> {
    pub database: &'a mut DataCenter,
    pub game: Game,
}

// todo - separate logic
impl<'a> OperationsCenter<'a> {
    // pub fn start_game(&self) {
    //     self.start_game()
    // }

    fn new_relationship(&mut self, nr: NewTaskRelationship) {
        // if self.database.does_compared_relationship_exist(&cr.winner_id)
        //     || self.database.does_compared_relationship_exist(&cr.looser_id) {
        //     let winning_relationship = ComparedRelationship {
        //         winner_id: if nr.winner_is_first { nr.first_id } else { nr.second_id },
        //         looser_id: if nr.winner_is_first { nr.second_id } else { nr.first_id },
        //         abstraction_level: nr.first_abstract_lvl + 1,
        //         reason: nr.reason,
        //     };
        //     self.database.insert_compared_relationships(winning_relationship);
        // } else {
        //     // todo - error
        // }
    }

    fn get_relationships_to_compare(&self) -> &ComparedRelationship {
        // self.compared_relationships.get(&id).is_some()
        // self.compare_with_abstract_level_0_task();
        todo!()
        // match self.database.relationship_stack.last() {
        //     None => self.database.compare,
        //     // None => ComparedRelationship {
        //     //     winner_id: 0,
        //     //     looser_id: 0,
        //     //     abstraction_level: 0,
        //     //     reason: "".to_string(),
        //     // },
        //     Some(compared_relationship) => compared_relationship
        // }
    }
}
