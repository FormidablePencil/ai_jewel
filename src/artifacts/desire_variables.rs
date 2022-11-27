pub struct DesireVariables {
    pub assumed_difficulty: Option<u8>,
    pub actual_difficulty: Option<u8>,
    pub assumed_reward: Option<u8>,
    pub actual_reward: Option<u8>,
    pub completed: bool,
    pub notes: String,
}

impl DesireVariables {
    fn get_roi_ration(&self) -> u8 {
        // todo - calculate for actual and assumed roi

        let mut assumed_roi = 1;
        let mut actual_roi = 1;

        let mut assumed_roi_failed = false;
        if self.assumed_reward.is_some() && self.assumed_difficulty.is_some() {
            assumed_roi = self.assumed_reward.unwrap() - self.assumed_difficulty.unwrap()
        } else {
            assumed_roi_failed = true
        }

        let mut actual_roi_failed = false;
        if self.actual_reward.is_some() && self.actual_difficulty.is_some() {
            actual_roi = self.actual_reward.unwrap() - self.actual_difficulty.unwrap()
        } else {
            assumed_roi_failed = true
        }

        assumed_roi / actual_roi
    }
}