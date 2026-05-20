use executor::{Pose,SportsCarExecutor as Executor};

mod sport_car_move_tests{
    use super::*;

    #[test]
    fn should_return_x_plus_2_given_command_is_m_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("M");
        // then
        let expected_pose = Pose::new(2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }
    
    #[test]
    fn should_return_x_mius_2_given_command_is_bm_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("BM");
        // then
        let expected_pose = Pose::new(-2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_4_given_command_is_fm_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("FM");
        // then
        let expected_pose = Pose::new(4, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }
    
    #[test]
    fn should_return_x_mius_4_given_command_is_bfm_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("BFM");
        // then
        let expected_pose = Pose::new(-4, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

}

mod sport_car_turn_left_tests{
   use super::*;

    #[test]
    fn should_return_y_plus_1_and_facing_n_given_command_is_l_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("L");
        // then
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
    
    #[test]
    fn should_returny_y_plus_1_and_facing_s_given_command_is_bl_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("BL");
        // then
        let expected_pose = Pose::new(0, 1, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_y_plus_1_and_facing_n_given_command_is_fl_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("FL");
        // then
        let expected_pose = Pose::new(1, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
    
    #[test]
    fn should_return_x_mius_1_y_plus_1_and_facing_s_given_command_is_fbl_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("BFL");
        // then
        let expected_pose = Pose::new(-1, 1, 'S');
        assert_eq!(expected_pose, executor.query());
    } 
}

mod sport_car_turn_right_tests{
   use super::*;

    #[test]
    fn should_return_y_mius_1_and_facing_s_given_command_is_r_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("R");
        // then
        let expected_pose = Pose::new(0, -1, 'S');
        assert_eq!(expected_pose, executor.query());
    }
    
    #[test]
    fn should_return_y_mius_1_and_facing_n_given_command_is_br_and_facing_is_e(){
         // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("BR");
        // then
        let expected_pose = Pose::new(0, -1, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_y_mius_1_and_facing_s_given_command_is_fr_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("FR");
        // then
        let expected_pose = Pose::new(1, -1, 'S');
        assert_eq!(expected_pose, executor.query());
    }
    
    #[test]
    fn should_return_x_mius_1_y_mius_1_and_facing_n_given_command_is_fbr_and_facing_is_e(){
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("BFR");
        // then
        let expected_pose = Pose::new(-1, -1, 'N');
        assert_eq!(expected_pose, executor.query());
    } 
}