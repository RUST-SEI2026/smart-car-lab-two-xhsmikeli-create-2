use std::io::{self, Write};

use executor::{Executor, Pose};

fn main() {
    // 初始化执行器：车辆从原点出发，默认朝北。
    let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));

    println!("Executor 的初始状态为 (0, 0, N).");
    println!("请输入命令序列，可包含 M、L、R、B、F，或输入 Q 退出程序。");
    println!("M: 前进/后退移动，L: 左转，R: 右转，B: 切换倒车状态，F: 切换加速状态。");
    print_pose(&executor);
    // 进入交互式命令输入循环，允许用户连续输入命令并查看结果。
    loop {
        print!("> ");

        //  确保输出被刷新，以便用户看到提示。
        if let Err(err) = io::stdout().flush() {
            eprintln!("刷新输出失败: {err}");
            break;
        }

        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("读取输入失败: {err}");
            break;
        }

        // trim() 去掉换行等空白字符，to_uppercase() 允许用户输入小写命令。
        let cmd = input.trim().to_uppercase();
        if cmd.is_empty() {
            continue;
        }
        // 单独输入 Q 时退出交互程序。
        if cmd == "Q" {
            println!("Over.");
            break;
        }
        // 检查命令是否仅包含 M、L、R、B、F。
        if !cmd
            .chars()
            .all(|ch| matches!(ch, 'M' | 'L' | 'R' | 'B' | 'F'))
        {
            println!("无效命令: {cmd}. 请输入仅包含 M、L、R、B、F 的命令序列，或输入 Q 退出程序。");
            continue;
        }

        // 执行整串命令，并在每次执行后展示当前位置和朝向。
        executor.execute(&cmd);
        print_pose(&executor);
    }
}

// 查询当前 Pose 状态并打印。
fn print_pose(executor: &Executor) {
    let pose = executor.query();
    println!("当前位置: ({}, {}, {})", pose.x, pose.y, pose.heading);
}
