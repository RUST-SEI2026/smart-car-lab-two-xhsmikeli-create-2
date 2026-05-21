use executor::{Executor, Pose};

// 验证 B 指令进入倒车状态后的 M/L/R 行为，以及再次输入 B 指令后取消倒车状态。
mod reverse_tests {
    use super::*;

    #[test]
    fn should_return_x_minus_1_given_status_is_reverse_command_is_m_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 M。
        executor.execute("BM");

        // 倒车状态下朝东执行 M 表示后退一格，因此 x 坐标应减 1。
        let expected_pose = Pose::new(-1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_s_given_status_is_reverse_command_is_l_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 L。
        executor.execute("BL");

        // 倒车状态下执行 L 表示右转，因此 E 应变为 S。
        let expected_pose = Pose::new(0, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_n_given_status_is_reverse_command_is_r_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 R。
        executor.execute("BR");

        // 倒车状态下执行 R 表示左转，因此 E 应变为 N。
        let expected_pose = Pose::new(0, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_bbm_and_facing_is_n() {
        // 初始位置在原点，车辆朝北方向。
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // 第一次 B 进入倒车状态，第二次 B 取消倒车状态，再执行 M。
        executor.execute("BBM");

        // 倒车状态被取消后，M 恢复为普通前进，因此 y 坐标应加 1。
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}
