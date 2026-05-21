use executor::{Executor, Pose}; 

// 验证多条命令连续执行时，状态切换和任务编排是否正确。
mod integration_tests {
    use super::*;

    #[test]
    fn should_return_y_plus_1_given_command_contains_forward_reverse_and_basic_move() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // M 普通前进，BM 倒车后退，B 取消倒车，L 左转，M 再普通前进。
        executor.execute("MBMBLM");

        // 在进行 MBMB 操作后，车辆应回到原点且为正常行驶状态，再执行 LM 操作后，方向朝北且 y 坐标应加 1。
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_3_and_n_given_command_contains_fast_cancel_and_basic_move() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // F 进入加速状态，M 在加速状态下前进两格，F 取消加速状态，L 左转，M 普通前进。
        executor.execute("FMFML");

        // 在执行 FMF 操作后，x 坐标应加 2 且为正常行驶状态，再执行 LM 操作后，方向朝北且 y 坐标应加 1。
        let expected_pose = Pose::new(3, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_2_and_s_given_command_contains_reverse_fast_and_cancel_fast() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // B 进入倒车状态，F 进入加速状态，M 在倒车加速状态下后退两格，F 取消加速状态，最后 L 左转（在倒车状态下原子操作为右转）。
        executor.execute("BFMFL");

        // 在执行 BFMF 操作后，x 坐标应减 2 且为后退行驶状态，再执行 L 操作后，方向朝南。
        let expected_pose = Pose::new(-2, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_3_y_plus_1_and_s_given_command_contains_multiple_state_switches() {
        // 初始位置在原点，车辆朝北方向。
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // M 普通前进，F 进入加速状态，R 右转，B 进入倒车状态，M 在倒车加速状态下后退一格，L 左转，B 取消倒车，F 进入加速状态，M 在加速状态下前进两格。
        executor.execute("MFRBMLBFM");

        // 按顺序执行，车辆最终应位于 (-3, 1)，朝向为 S。
        let expected_pose = Pose::new(-3, 1, 'S');
        assert_eq!(expected_pose, executor.query());
    }
}
