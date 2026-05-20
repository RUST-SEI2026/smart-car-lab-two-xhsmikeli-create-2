use crate::Pose;

// Action 作为原子操作，把复杂业务指令拆成最小可执行单元，其中包括
// - Forward(i32)：向前或向后移动一步，可以叠加
// - TurnLeft：左转
// - TurnRight：右转
// 例如：
// - FM  会被拆分成 Forward(1), Forward(1)
// - BFL 会被拆分成 Forward(-1), TurnRight
#[derive(Copy, Clone)]
pub(crate) enum Action {
    Forward(i32),
    TurnLeft,
    TurnRight,
}
// 将一个原子动作真正作用到 Pose 上。
impl Action {
    // Action::perform 负责调用 Pose 完成实际坐标或朝向变化。
    pub(crate) fn perform(&self, pose: &mut Pose) {
        match self {
            Self::Forward(offset) => pose.forward(*offset), // 调用 Pose 的 forward 方法，*offset 解引用获取 i32 值。
            Self::TurnLeft => pose.turn_left(),             // 调用 Pose 的 turn_left 方法
            Self::TurnRight => pose.turn_right(),           // 调用 Pose 的 turn_right 方法
        }
    }
}
