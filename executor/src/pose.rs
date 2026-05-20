// Pose 表示车辆在二维坐标系中的位置和朝向。
// x 轴：向东为正，向西为负。
// y 轴：向北为正，向南为负。
// heading 使用 E/S/W/N 四个字符表示东、南、西、北四个方向。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}
impl Pose {
    // 创建一个指定坐标和朝向的车辆状态。
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Self { x, y, heading }
    }

    // 按当前朝向移动 offset 格。
    // 例如 heading 为 E：
    // - forward(1)  表示 x + 1
    // - forward(-1) 表示 x - 1
    pub(crate) fn forward(&mut self, offset: i32) {
        match self.heading {
            'E' => self.x += offset,
            'S' => self.y -= offset,
            'W' => self.x -= offset,
            'N' => self.y += offset,
            _ => {}
        }
    }

    // 左转 90 度，改变朝向。
    pub(crate) fn turn_left(&mut self) {
        self.heading = match self.heading {
            'E' => 'N',
            'S' => 'E',
            'W' => 'S',
            'N' => 'W',
            heading => heading,
        };
    }

    // 右转 90 度，改变朝向。
    pub(crate) fn turn_right(&mut self) {
        self.heading = match self.heading {
            'E' => 'S',
            'S' => 'W',
            'W' => 'N',
            'N' => 'E',
            heading => heading,
        };
    }
}

impl Default for Pose {
    // 初始化车辆状态为 (0, 0, 'N')，且朝向北。
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}
