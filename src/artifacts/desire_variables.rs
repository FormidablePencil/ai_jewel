pub struct DesireVariables {
    pub assumed_difficulty: Option<u8>,
    pub actual_difficulty: Option<u8>,
    pub assumed_reward: Option<u8>,
    pub actual_reward: Option<u8>,
    pub completed: bool,
    pub notes: String,
}

pub enum GetRoiRationError {
    NeitherAssumedNorActualVariablesExists,
}

pub enum RoiExists {
    OnlyAssumedRoiVarExist,
    OnlyActualRoiVarExist,
    BothAssumedAndActualRoiVarExist,
    NoRoiVarExist,
}

impl DesireVariables {
    fn do_assumed_variables_exist(&self) -> RoiExists {
        let actual_roi_var_exists =
            self.actual_reward.is_some() && self.actual_difficulty.is_some();
        let assumed_roi_var_exists =
            self.assumed_reward.is_some() && self.assumed_difficulty.is_some();

        if !assumed_roi_var_exists && actual_roi_var_exists {
            RoiExists::OnlyAssumedRoiVarExist
        } else if assumed_roi_var_exists && !actual_roi_var_exists {
            RoiExists::OnlyActualRoiVarExist
        } else if assumed_roi_var_exists && actual_roi_var_exists {
            RoiExists::BothAssumedAndActualRoiVarExist
        } else {
            todo!("Should never be hit!")
        }
    }

    pub fn get_roi(&self) -> Option<u128> {
        match self.do_assumed_variables_exist() {
            RoiExists::OnlyAssumedRoiVarExist => {
                Some((self.assumed_reward.unwrap() - self.assumed_difficulty.unwrap()) as u128)
            }
            RoiExists::OnlyActualRoiVarExist => {
                Some((self.actual_reward.unwrap() - self.actual_difficulty.unwrap()) as u128)
            }
            RoiExists::BothAssumedAndActualRoiVarExist => Some(
                ((self.assumed_reward.unwrap() - self.assumed_difficulty.unwrap())
                    / (self.actual_reward.unwrap() - self.actual_difficulty.unwrap()))
                    as u128,
            ),
            // todo - logging the issue would be useful
            RoiExists::NoRoiVarExist => None,
        }
    }
}
