use executor::{Executor, Pose}; 

// 验证 F 指令进入加速状态后的 M/L/R 行为，以及再次输入 F 指令后取消加速状态。
mod fast_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_2_given_status_is_fast_command_is_m_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 进入加速状态，再执行 M。
        executor.execute("FM");

        // 加速状态下 M 表示前进两格，因此 x 坐标应加 2。
        let expected_pose = Pose::new(2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_and_n_given_status_is_fast_command_is_l_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 进入加速状态，再执行 L。
        executor.execute("FL");

        // 加速状态下 L 表示先前进一格，再左转。
        let expected_pose = Pose::new(1, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_and_s_given_status_is_fast_command_is_r_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 进入加速状态，再执行 R。
        executor.execute("FR");

        // 加速状态下 R 表示先前进一格，再右转。
        let expected_pose = Pose::new(1, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_ffm_and_facing_is_n() {
        // 初始位置在原点，车辆朝北方向。
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // 第一次 F 进入加速状态，第二次 F 取消加速状态，再执行 M。
        executor.execute("FFM");

        // 加速状态被取消后，M 恢复为普通前进，因此 y 坐标应加 1。
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}
