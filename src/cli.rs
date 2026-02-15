use std::env;        // env::args：读取命令行参数
use std::fs;         // fs::read：读取文件
use std::path::Path; // Path：更语义化的路径处理

use crate::stats::{CountMode, FileStats}; // crate::：引用当前 crate 的其它模块导出

/// run：把 CLI 的“主流程”放到这里，方便 main.rs 只负责打印错误
pub fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    // env::args()：返回 Iterator<Item=String>
    // collect()：把迭代器收集成 Vec<String>
    // 显式类型 Vec<String>：帮助你理解 args 是拥有所有权的字符串集合

    let (mode, file) = parse_args(&args)?;
    // (mode, file)：用元组接收返回值
    // parse_args(&args)：借用传参，避免复制
    // ?：如果 Err，立刻向上传播（run 返回 Err）

    let path = Path::new(&file);
    // Path::new(&file)：把 String 借用为 &str，把一个字符串切片（ &str ）包装成一个路径切片（ &Path ）
    // 注意：Path 本身不拥有文件名，只是引用

    let bytes = fs::read(path).map_err(|e| format!("cannot read {file}: {e}"))?;
    // fs::read：读取整个文件到 Vec<u8>
    // map_err：把 std::io::Error 转成我们统一的 String 错误
    // ?：传播错误

    let stats = FileStats::from_bytes(&bytes);
    // from_bytes：只借用 &[u8]，不 move bytes（bytes 仍可用）
    // 返回一个统计结构体（拥有统计结果）

    print_output(&stats, mode, &file);
    // &stats：借用统计结果
    // &file：借用文件名（避免 clone）

    Ok(())
}

/// parse_args：把命令行参数解析成 (统计模式, 文件名)
fn parse_args(args: &[String]) -> Result<(CountMode, String), String> {
    // args: &[String]：切片借用（比 &Vec<String> 更通用）
    // 返回 String：文件名需要拥有所有权（后面要长期使用）

    if args.len() <= 1 {
        // args[0] 通常是程序名，因此 <=1 表示没有提供文件参数
        return Err(help_message("missing file path"));
    }

    // 支持：mini_wc file.txt
    // 支持：mini_wc -l file.txt
    // 支持：mini_wc --help
    let mut mode = CountMode::All; // 默认：输出全部
    let mut file: Option<String> = None;
    // Option：表示“可能没有拿到文件名”
    // 这里 file = None 表示还没解析到

    // 跳过 args[0]（程序名），从后面的用户参数开始解析
    let mut it = args.iter().skip(1);
    // args.iter()：产生 Iterator<Item=&String>（借用迭代）
    // skip(1)：跳过第一个参数

    while let Some(arg) = it.next() {
        // while let：模式匹配循环
        // it.next()：返回 Option<&String>
        let s = arg.as_str();
        // as_str：把 &String 转成 &str（借用视图）

        match s {
            "--help" | "-h" => return Err(help_message("")),
            "-l" => mode = CountMode::Lines,
            "-w" => mode = CountMode::Words,
            "-c" => mode = CountMode::Bytes,
            "-m" => mode = CountMode::Chars,
            _ => {
                // 非 flag：当作文件路径
                // 这里把 &String 克隆成 String（拥有所有权），因为 arg 是借用
                file = Some(arg.clone());
            }
        }
    }

    let file = file.ok_or_else(|| help_message("missing file path"))?;
    // ok_or_else：把 Option 转 Result
    // ok_or_else 闭包：延迟构造错误字符串
    // ?：传播 Err

    Ok((mode, file))
}

/// 输出逻辑：按 mode 打印对应字段
fn print_output(stats: &FileStats, mode: CountMode, file: &str) {
    // stats: &FileStats：借用
    // mode: CountMode：小枚举，按值传没问题
    // file: &str：借用字符串切片（更通用）

    match mode {
        CountMode::All => {
            println!(
                "lines={} words={} bytes={} chars={}  {}",
                stats.lines, stats.words, stats.bytes, stats.chars, file
            );
        }
        CountMode::Lines => println!("lines={}  {}", stats.lines, file),
        CountMode::Words => println!("words={}  {}", stats.words, file),
        CountMode::Bytes => println!("bytes={}  {}", stats.bytes, file),
        CountMode::Chars => println!("chars={}  {}", stats.chars, file),
    }
}

/// help_message：统一生成帮助信息（用 Err 返回，让 main.rs 打印）
fn help_message(reason: &str) -> String {
    // reason: &str：借用，适合常量/拼接
    let mut msg = String::new(); // 创建可变 String，逐步 push_str
    if !reason.is_empty() {
        msg.push_str(reason);
        msg.push('\n');
    }
    msg.push_str(
        "Usage:
  mini_wc <file>
  mini_wc -l <file>   # lines
  mini_wc -w <file>   # words
  mini_wc -c <file>   # bytes
  mini_wc -m <file>   # chars (unicode)
  mini_wc --help
",
    );
    msg
}
