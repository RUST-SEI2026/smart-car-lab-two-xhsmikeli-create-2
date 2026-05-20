use crate::{Action, Pose, State};

// Executor 表示车辆指令执行器，内部保存两部分状态：
// - pose：车辆当前所在位置和朝向。
// - state：车辆当前是否处于倒车状态、加速状态。

pub struct Executor {
    pose: Pose,
    state: State,
}

impl Executor {
    // with_pose() 是构造函数，允许调用者指定车辆初始位置和朝向。
    pub fn with_pose(pose: Pose) -> Self {
        Self {
            pose,
            state: State::default(),
        }
    }

    // 构造函数，用来创建指定位置和朝向的 Pose。
    pub fn new(pose: Pose) -> Self {
        Self::with_pose(pose)
    }

    // execute() 会逐个读取字符串中的字符，并按照命令类型分别处理：
    // - B：切换倒车状态。
    // - F：切换加速状态。
    // - M/L/R：交给 State 生成 Action 序列，再统一执行。
    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B' => self.state.be_reverse(),
                'F' => self.state.be_fast(),
                _ => self.perform(self.state.assemble(cmd)),
            }
        }
    }

    // query() 用来查询当前车辆状态。
    pub fn query(&self) -> Pose {
        self.pose
    }

    // perform() 负责执行 State 编排出来的原子动作序列。
    fn perform(&mut self, actions: Vec<Action>) {
        for action in actions {
            action.perform(&mut self.pose); // 将当前原子动作作用到Pose上，更新坐标或朝向。
        }
    }
}
// Executor 实现了 Default trait，允许使用默认构造函数创建初始状态为(0, 0, N) 的执行器。
impl Default for Executor {
    fn default() -> Self {
        Self::with_pose(Pose::default())
    }
}
