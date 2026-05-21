use executor::{Executor, Pose};

// 验证 B 和 F 两个状态叠加后的 M/L/R 行为。
mod reverse_fast_tests {
    use super::*;

    #[test]
    fn should_return_x_minus_2_given_status_is_reverse_and_fast_command_is_m_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 F 进入加速状态，最后执行 M。
        executor.execute("BFM");

        // 倒车加速状态下 M 表示后退两格，因此 x 坐标应减 2。
        let expected_pose = Pose::new(-2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_and_s_given_status_is_reverse_and_fast_command_is_l_and_facing_is_e()
    {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 F 进入加速状态，最后执行 L。
        executor.execute("BFL");

        // 倒车加速状态下 L 表示先倒退一格，再右转，因此 x 坐标应减 1，朝向应变为 S。
        let expected_pose = Pose::new(-1, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_and_n_given_status_is_reverse_and_fast_command_is_r_and_facing_is_e()
    {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 F 进入加速状态，最后执行 R。
        executor.execute("BFR");

        // 倒车加速状态下 R 表示先倒退一格，再左转，因此 x 坐标应减 1，朝向应变为 N。
        let expected_pose = Pose::new(-1, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}
