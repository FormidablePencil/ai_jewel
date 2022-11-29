use crate::artifacts::ai_gen::MyTask;
use crate::artifacts::attachments::FuckedUp;
use crate::artifacts::desire_variables::DesireVariables;
use crate::artifacts::operations::ComparedRelationship;
use std::collections::HashMap;

pub struct TaskRank {
    pub roi: u128,
    pub task_id: u128,
    // pub(crate) notes: String,
}

pub struct DataCenter {
    pub alive: bool,
    pub fucked_up: Vec<FuckedUp>,
    pub unique_index: u128,
    pub tasks: HashMap<u128, MyTask>,
    pub desire_variables: HashMap<u128, DesireVariables>,
    // pub tasks_rank: SortedList<u128, TaskRank>,
    pub tasks_rank: Vec<TaskRank>,
    // <task_id, index of task_rank>
    pub tr_index: HashMap<u128, u128>,
    // <index of task_rank, task_id>
    pub tr_index_reverse: HashMap<u128, u128>,

    // pub relationship_uniqueselfindex: u128,
    pub compared_relationships: HashMap<u128, ComparedRelationship>,
    pub relationship_stack: Vec<ComparedRelationship>,
}

impl DataCenter {
    pub fn default_data_center() -> DataCenter {
        DataCenter {
            unique_index: 0,
            tasks: HashMap::new() as HashMap<u128, MyTask>,
            desire_variables: HashMap::new() as HashMap<u128, DesireVariables>,
            tasks_rank: vec![],
            tr_index: Default::default(),
            tr_index_reverse: Default::default(),
            compared_relationships: Default::default(),
            alive: true,
            fucked_up: vec![],
            relationship_stack: vec![],
        }
    }

    pub fn get_available_roi(&self, roi: u128) {
        // let f = self.tasks_rank.get_();
        // .unwrap();

        // for self.tasks_rank.iter().enumerate() {
        //
        // }
        // // (&roi);
        // let f = f.unwrap();
        // self.tasks_rank.insert(roi);
        // let next_roi =
        //
        // self.tasks_rank.(&roi);
    }

    pub fn insert_tasks_rank(&mut self, task_rank: TaskRank) {
        // let res = self.tasks_rank.insert(TaskRank{
        //     task_id: task_rank.task_id,
        //     roi: self.get_available_roi(task_rank.roi)
        // });
        // // should never happen but just in case.
        // if !res { self.fucked_up.push(FuckedUp::standard("Failed to insert a new task rank roi.".to_string())) }
    }

    pub fn insert_fucked_up(&mut self, fucked_up: FuckedUp) {
        self.fucked_up.push(fucked_up)
    }

    pub fn increment_unique_index(&mut self) {
        self.unique_index = self.unique_index + 1
    }

    pub fn insert_task(&mut self, unique_index: u128, task: MyTask) {
        self.tasks.insert(unique_index, task);
    }

    pub fn insert_desire_variables(&mut self, desire_variables: DesireVariables) {
        self.desire_variables
            .insert(self.unique_index, desire_variables);
    }

    pub fn insert_compared_relationships(&mut self, winning_relationship: ComparedRelationship) {
        self.compared_relationships
            .insert(self.unique_index, winning_relationship);
    }

    pub fn delete_task(&mut self, index: u128) -> MyTask {
        self.desire_variables.remove(&index);
        match self.tasks.remove(&index) {
            None => todo!("return error to me..."),
            Some(task) => task,
        }
    }

    pub fn does_task_exist(&self, task_id: u128) -> bool {
        // todo - handle desire_variables = self.desire_variables.get(&task_id).is_some();
        self.tasks.get(&task_id).is_some()
    }

    pub fn does_compared_relationship_exist(&mut self, id: &u128) -> bool {
        self.compared_relationships.get(&id).is_some()
    }
}

#[cfg(test)]
mod data_center_tests {
    use super::*;

    #[test]
    fn does_task_exist() {
        let mut data_center = DataCenter::default_data_center();
        // data_center.insert_task();
        // data_center.does_compared_relationship_exist(&123);
        // assert_eq!(result, 4);
    }
}
