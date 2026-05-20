use crate::Action;

// State 记录执行器当前的两个运动标志，是否为倒车状态，是否为加速状态。
// - is_reverse: 倒车状态
// - is_fast: 加速状态
#[derive(Default, Copy, Clone)]
pub(crate) struct State {
    is_reverse: bool,
    is_fast: bool,
}

impl State {
    // 切换倒车状态。
    // 第一次 B 标志从 false 反转为 true，进入倒车状态；第二次 B 标志从 true 反转为 false，恢复正常状态。
    pub(crate) fn be_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    // 切换加速状态。
    // 第一次 F 标志从 false 反转为 true，进入加速状态；第二次 F 标志从 true 反转为 false，恢复正常状态。
    pub(crate) fn be_fast(&mut self) {
        self.is_fast = !self.is_fast;
    }

    // 根据当前状态，把控制指令 M/L/R 编排成原子动作序列，函数返回 Vec<Action>，表示计划执行的一组动作。
    pub(crate) fn assemble(&self, cmd: char) -> Vec<Action> {
        match cmd {
            'M' => self.move_assemble(),
            'L' => self.turn_left_assemble(),
            'R' => self.turn_right_assemble(),
            _ => Vec::new(),
        }
    }

    // 编排 M 指令，共四种分别为：
    // 1. 普通状态：Forward(1)
    // 2. 倒车状态：Forward(-1)
    // 3. 加速状态：Forward(1), Forward(1)
    // 4. 倒车加速：Forward(-1), Forward(-1)
    fn move_assemble(&self) -> Vec<Action> {
        let step_count = if self.is_fast { 2 } else { 1 };

        vec![Action::Forward(self.direction()); step_count]
    }

    // 编排 L 指令，共四种分别为：
    // 1. 普通状态：TurnLeft
    // 2. 倒车状态：TurnRight
    // 3. 加速状态：Forward(1), TurnLeft
    // 4. 倒车加速：Forward(-1), TurnRight
    fn turn_left_assemble(&self) -> Vec<Action> {
        let mut actions = self.fast_prefix();
        actions.push(if self.is_reverse {
            Action::TurnRight
        } else {
            Action::TurnLeft
        });
        actions
    }

    // 编排 R 指令，共四种分别为：
    // 1. 普通状态：TurnRight
    // 2. 倒车状态：TurnLeft
    // 3. 加速状态：Forward(1), TurnRight
    // 4. 倒车加速：Forward(-1), TurnLeft
    fn turn_right_assemble(&self) -> Vec<Action> {
        let mut actions = self.fast_prefix();
        actions.push(if self.is_reverse {
            Action::TurnLeft
        } else {
            Action::TurnRight
        });
        actions
    }

    // 加速状态下，L/R 在转向前需要先移动一格。
    fn fast_prefix(&self) -> Vec<Action> {
        if self.is_fast {
            vec![Action::Forward(self.direction())]
        } else {
            Vec::new()
        }
    }

    // 当前移动方向。
    //  1. 前进状态：1
    //  2. 倒车状态：-1
    fn direction(&self) -> i32 {
        if self.is_reverse {
            -1
        } else {
            1
        }
    }
}
