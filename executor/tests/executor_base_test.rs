use executor::{Executor, Pose}; // 引入被测试对象 Executor 和 Pose。

// 验证 M 指令在不同朝向下的移动行为。
mod move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次基础移动指令 M。
        executor.execute("M");

        // 朝东前进一格后，x 坐标应加 1，朝向保持 E。
        let expected_pose = Pose::new(1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn shoud_return_y_minus_1_given_command_is_m_and_facing_is_s() {
        // 初始位置在原点，车辆朝南方向。
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次基础移动指令 M。
        executor.execute("M");

        // 朝南前进一格后，y 坐标应减 1，朝向保持 S。
        let expected_pose = Pose::new(0, -1, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn shoud_return_x_minus_1_given_command_is_m_and_facing_is_w() {
        // 初始位置在原点，车辆朝西方向。
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次基础移动指令 M。
        executor.execute("M");

        // 朝西前进一格后，x 坐标应减 1，朝向保持 W。
        let expected_pose = Pose::new(-1, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn shoud_return_y_plus_1_given_command_is_m_and_facing_is_n() {
        // 初始位置在原点，车辆朝北方向。
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次基础移动指令 M。
        executor.execute("M");

        // 朝北前进一格后，y 坐标应加 1，朝向保持 N。
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}

// 验证 L 指令在不同朝向下的左转行为。
mod turn_left_tests {
    use super::*;

    #[test]
    fn should_return_facing_n_given_command_is_l_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次左转指令 L。
        executor.execute("L");

        // 朝东左转后应朝北，位置不变。
        let expected_pose = Pose::new(0, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_e_given_command_is_l_and_facing_is_s() {
        // 初始位置在原点，车辆朝南方向。
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次左转指令 L。
        executor.execute("L");

        // 朝南左转后应朝东，位置不变。
        let expected_pose = Pose::new(0, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_s_given_command_is_l_and_facing_is_w() {
        // 初始位置在原点，车辆朝西方向。
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次左转指令 L。
        executor.execute("L");

        // 朝西左转后应朝南，位置不变。
        let expected_pose = Pose::new(0, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_w_given_command_is_l_and_facing_is_n() {
        // 初始位置在原点，车辆朝北方向。
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次左转指令 L。
        executor.execute("L");

        // 朝北左转后应朝西，位置不变。
        let expected_pose = Pose::new(0, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }
}

// 验证 R 指令在不同朝向下的右转行为。
mod turn_right_tests {
    use super::*;

    #[test]
    fn should_return_facing_s_given_command_is_r_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次右转指令 R。
        executor.execute("R");

        // 朝东右转后应朝南，位置不变。
        let expected_pose = Pose::new(0, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_w_given_command_is_r_and_facing_is_s() {
        // 初始位置在原点，车辆朝南方向。
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次右转指令 R。
        executor.execute("R");

        // 朝南右转后应朝西，位置不变。
        let expected_pose = Pose::new(0, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_n_given_command_is_r_and_facing_is_w() {
        // 初始位置在原点，车辆朝西方向。
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次右转指令 R。
        executor.execute("R");

        // 朝西右转后应朝北，位置不变。
        let expected_pose = Pose::new(0, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_e_given_command_is_r_and_facing_is_n() {
        // 初始位置在原点，车辆朝北方向。
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次右转指令 R。
        executor.execute("R");

        // 朝北右转后应朝东，位置不变。
        let expected_pose = Pose::new(0, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }
}
