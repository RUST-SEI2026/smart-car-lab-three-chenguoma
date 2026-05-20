use crate::Pose;
use crate::executor::executor::Executor;
use crate::assembler::sport_car_state::SportsCarState;

pub struct SportsCarExecutor;

impl SportsCarExecutor{
    pub fn with_pose(pose:Pose) -> Executor{
        Executor{
            pose,
            state: Box::new(SportsCarState::default()),
        } 
    }

}