use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

use std::fs::File;
use std::os::unix::io::AsRawFd;

mod models;
#[allow(unused)]
use models::{my_struct::*, func_koopa::*};

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

#[allow(unused)]
fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;
  let file = File::create(output)?;
  let raw_fd = file.as_raw_fd();
    unsafe {
        libc::dup2(raw_fd, libc::STDOUT_FILENO);
    }


  // 调用 lalrpop 生成的 parser 解析输入文件
  let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

  // 输出解析得到的 AST
  println!("{}", ast.tree());
  Ok(())
}
