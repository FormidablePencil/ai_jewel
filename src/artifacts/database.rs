use std::borrow::Borrow;
use std::collections::HashMap;
use crate::artifacts::ai_gen::MyTask;
use crate::artifacts::desire_variables::DesireVariables;
use crate::artifacts::operations::{CompareRelationship, ComparedRelationship, NewRelationship};

pub struct Database {
    pub alive: bool,
    pub unique_index: u128,
    pub tasks: HashMap<u128, MyTask>,
    pub desire_variables: HashMap<u128, DesireVariables>,

    // pub relationship_unique_index: u128,
    pub compared_relationships: HashMap<u128, ComparedRelationship>,
    pub relationship_stack: Vec<ComparedRelationship>,
}

impl Database {
    pub fn add_task(mut self, task: MyTask, desire_variables: DesireVariables) -> bool {

        return if self.desire_variables.contains_key(unique_index) == false
            || self.tasks.contains_key(unique_index) == false {

            self.unique_index = self.unique_index + 1;
            self.tasks.insert(self.unique_index, task);
            self.desire_variables.insert(self.unique_index, desire_variables);

            true

        } else { false }
    }

    pub fn delete_task(&mut self, index: u128) -> MyTask {
        self.desire_variables.remove(&index);
        match self.tasks.remove(&index) {
            None => todo!("return error to me..."),
            Some(task) => task
        }
    }

    pub fn compare_tasks(&self, task1_relationship_id: u128, task2_relationship_id: u128) {
        // todo - error handling
        let task1_desire_var = self.desire_variables.get(&task1_relationship_id).expect("");
        let task2_desire_var = self.desire_variables.get(&task2_relationship_id).expect("");

        if task1_desire_var.assumed_difficulty > task2_desire_var.assumed_difficulty {

        }
    }

    pub fn compare_with_abstract_level_0_task(&self, rn: NewRelationship) {
        if !self.does_task_exist(rn.first_id) {
            todo!("task of provided id doesn't exist.")
        } else if !self.does_task_exist(rn.second_id) {
            todo!("task of provided id doesn't exist.")
        } else if rn.first_abstract_lvl == 0 {
            self.tasks.get(&rn.first_id);
        }
        self.compared_relationships.get(rn.first_id)

        todo!()
    }

    fn does_task_exist(&self, task_id: u128) -> bool {
        self.tasks.get(&task_id).is_some()
    }

    fn does_compared_relationship_exist(&mut self, id: &u128) -> bool {
        self.compared_relationships.get(&id).is_some()
    }
    // pub fn compare_with_abstract_level_0_task(&mut self, id: &u128) -> bool {
    // }

    pub fn new_relationship(&mut self, nr: NewRelationship) {
        if self.does_compared_relationship_exist(&cr.winner_id)
            || self.does_compared_relationship_exist(&cr.looser_id) {
            let winning_relationship = ComparedRelationship {
                winner_id: if nr.winner_is_first { nr.first_id } else { nr.second_id },
                looser_id: if nr.winner_is_first { nr.second_id } else { nr.first_id },
                abstraction_level: nr.first_abstract_lvl + 1,
                reason: nr.reason,
            };
            self.compared_relationships.insert(self.unique_index, winning_relationship);
        } else {
            // todo - error
        }
    }

    pub fn get_relationships_to_compare(&self) -> &ComparedRelationship {
        // self.compared_relationships.get(&id).is_some()
        // self.compare_with_abstract_level_0_task();
        // todo
        match self.relationship_stack.last() {
            None => self.compare,
            // None => ComparedRelationship {
            //     winner_id: 0,
            //     looser_id: 0,
            //     abstraction_level: 0,
            //     reason: "".to_string(),
            // },
            Some(compared_relationship) => compared_relationship
        }
    }
}
