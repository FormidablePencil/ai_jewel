use crate::artifacts::ai_gen::MyTask;
use crate::artifacts::attachments::FuckedUp;
use crate::artifacts::data_center::TaskRank;
use crate::artifacts::desire_variables::{DesireVariables, RoiExists};
use crate::artifacts::operations::{ComparedRelationship, NewTaskRelationship, OperationsCenter};

trait TasksOperations {
    // check if tasks exist
    fn do_tasks_exist(&mut self, task1_id: u128, task2_id: u128) -> CompareTasksRes;
    fn validate_roi_sameness(&mut self, desired_var1_roi: u8, desired_var2_roi: u8) -> CompareTasksRes;
    fn validate_both_roi_of_tasks_exist(&self, task1_id: u128, task2_id: u128) -> CompareTasksRes;
    fn rank_tasks(&self, task1_id: u128, task2_id: u128);
    fn add_task(mut self, task: MyTask, desire_variables: DesireVariables) -> bool;
    fn rank_task(&self, task1_id: u128, task2_id: u128);
    fn compare_with_abstract_level_0_task(&self, rn: NewTaskRelationship);
}

pub enum CompareTasksRes {
    Successful,
    IdOfTask1DoesNotExist,
    IdOfTask2DoesNotExist,
    NoRoiForTask1,
    NoRoiForTask2,
    NoRoiForTaskBoth,
    BothTasksRoiNotSame,
    BothTasksContainSameRoi,
}

impl<TaskExample, DesireVariables> TasksOperations for OperationsCenter<TaskExample, DesireVariables> {
    fn do_tasks_exist(&mut self, task1_id: u128, task2_id: u128) -> CompareTasksRes {
        let task1_exists = self.database.does_task_exist(task1_id);
        let task2_exists = self.database.does_task_exist(task2_id);

        if !task1_exists {
            CompareTasksRes::IdOfTask1DoesNotExist
        } else if !task2_exists {
            CompareTasksRes::IdOfTask2DoesNotExist
        } else {
            CompareTasksRes::Successful
        }
    }

    fn validate_roi_sameness(&mut self, task_winner_id: u128, task_looser_id: u128,) -> bool {
        let winner_roi = self.database.desire_variables.get(&task_winner_id).unwrap().get_roi();
        let looser_roi = self.database.desire_variables.get(&task_looser_id).unwrap().get_roi();
        winner_roi == looser_roi
    }

    fn validate_both_roi_of_tasks_exist(&self, task1_id: u128, task2_id: u128) -> CompareTasksRes {
        let desired_var1_roi = self.database.desire_variables.get(&task1_id).unwrap().get_roi();
        let desired_var2_roi = self.database.desire_variables.get(&task2_id).unwrap().get_roi();

        if desired_var1_roi.is_some() && desired_var2_roi.is_some() {
            CompareTasksRes::Successful
        } else if !desired_var1_roi.is_some() && desired_var2_roi.is_some() {
            CompareTasksRes::NoRoiForTask1
        } else if desired_var1_roi.is_some() && !desired_var2_roi.is_some() {
            CompareTasksRes::NoRoiForTask2
        } else if !desired_var1_roi.is_some() && !desired_var2_roi.is_some() {
            CompareTasksRes::NoRoiForTaskBoth
        }
    }

    /*
        - Checks that both loosing and winning tasks exist under id given
        - Checks that both ROI's of tasks are the same
        - Validates roi sameness
            true:
                - insert task rank

            false: return CompareTasksRes::BothTasksRoiNotSame
     */
    fn rank_tasks(&mut self, task_winner_id: u128, task_looser_id: u128) -> CompareTasksRes {
        let do_tasks_exist = self.do_tasks_exist(task_winner_id, task_looser_id);
        if do_tasks_exist != CompareTasksRes::Successful { do_tasks_exist }

        // validate if roi for ranking tasks exists
        match self.validate_both_roi_of_tasks_exist() {
            CompareTasksRes::Successful => {
                if self.validate_roi_sameness(task_winner_id, task_looser_id) {
                    let winner_roi = self.database.desire_variables.get(&task_winner_id).unwrap().get_roi().unwrap(); // already validated that desire_variable under task_winner_id and roi of it exists.
                    self.database.insert_tasks_rank(TaskRank { task_id: task_winner_id, roi: winner_roi });
                    CompareTasksRes::BothTasksContainSameRoi
                } else {
                    CompareTasksRes::BothTasksRoiNotSame
                }
            }
            CompareTasksRes::NoRoiForTask1 => CompareTasksRes::NoRoiForTask1,
            CompareTasksRes::NoRoiForTask2 => CompareTasksRes::NoRoiForTask2,
            CompareTasksRes::NoRoiForTaskBoth => {}

            CompareTasksRes::IdOfTask1DoesNotExist => {}
            CompareTasksRes::IdOfTask2DoesNotExist => {}

            CompareTasksRes::BothTasksRoiNotSame => {}
            CompareTasksRes::BothTasksContainSameRoi => {}
        }
    }

    fn add_task(&mut self, task: MyTask, desire_variables: DesireVariables) -> bool {
        return if self.database.desire_variables.contains_key(unique_index) == false
            || self.database.tasks.contains_key(unique_index) == false {
            self.database.increment_unique_index();
            self.database.insert_task(self.unique_index, task);
            self.database.insert_desire_variables(desire_variables);

            true
        } else { false };
    }

    fn rank_task(&self, task1_id: u128, task2_id: u128) {
        if !self.database.does_task_exist(task1_id) && !self.does_task_exist(task1_id) {
            // Error
        }

        let task1_desire_variables = self.database.desire_variables.get(&task1_id).unwrap().get_roi();

        match task1_desire_variables {
            Ok(roi) => roi,
            Err(_) => {}
        }
        let task2_desire_variables = self.database.desire_variables.get(&task2_id).unwrap().get_roi();
    }

    fn compare_with_abstract_level_0_task(&self, rn: NewTaskRelationship) {
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
}
