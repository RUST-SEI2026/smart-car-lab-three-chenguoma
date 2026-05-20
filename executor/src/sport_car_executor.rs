use crate::Pose;
use crate::executor::Executor;
use crate::sport_car_state::SportsCarState;

pub struct SportsCarExecutor;

impl SportsCarExecutor{
    pub fn with_pose(pose:Pose) -> Executor{
        Executor{
            pose,
            state: Box::new(SportsCarState::default()),
        } 
    }

// pub fn execute(&mut self, cmds: &str) {
//         for cmd in cmds.chars() {
//             match cmd {
//                 'B' => self.State.be_reverse(),
//                 'F' => self.state.be_fast(),
//                 _ => {
//                     let astions = self.state.assemble(cmd);
//                     for action in astions {
//                         action.perform(&mut self.pose)
//                     }
//                 }
//             }
//         }
//     }

    // pub fn query(&self) -> Pose{
    //     self.pose
    // }
}