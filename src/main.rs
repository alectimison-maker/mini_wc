mod cli;   // mod：声明模块 cli，对应 src/cli.rs
mod stats; // 声明模块 stats，对应 src/stats.rs

use std::process; // 引入进程退出工具（process::exit）

fn main() {
    // main 不返回 Result 的写法：我们手动 match 处理错误并输出
    match cli::run() {
        Ok(()) => {} // Ok(())：成功结束，不做任何事
        Err(e) => {
            eprintln!("Error: {e}"); // eprintln!：输出到 stderr（错误输出）
            process::exit(1);        // 非 0 退出码：告诉 shell 失败
        }
    }
}
