//! SBI console driver, for text output
//! 控制台驱动模块，用于文本输出
//! 本模块实现了基于SBI调用的控制台输出功能，提供了print和println宏，方便内核中的文本输出
//! SBI (Supervisor Binary Interface)是RISC-V架构中特权级之间的接口规范

use crate::sbi::console_putchar;
use core::fmt::{self, Write};

/// Stdout结构体，实现了Write trait，用于格式化输出
/// 这是一个单例结构体，不存储任何状态，仅用于实现Write trait
struct Stdout;

/// 为Stdout实现Write trait，以便能够进行格式化输出
/// Write trait是Rust标准库中用于输出的核心trait
impl Write for Stdout {
    /// 实现write_str方法，将字符串转换为字符并逐个输出
    /// 通过SBI调用将每个字符发送到控制台
    /// 
    /// # 参数
    /// * `s` - 要输出的字符串
    /// 
    /// # 返回值
    /// * `fmt::Result` - 输出操作的结果，始终为Ok(())
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

/// 打印函数，接收格式化参数并输出到控制台
/// 这是print和println宏的核心实现，将格式化参数传递给Stdout进行处理
/// 
/// # 参数
/// * `args` - 格式化参数，由format_args!宏创建
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// Print!宏，用于向主机控制台输出格式化文本
/// 接收格式化字符串和可选参数，与标准库print!宏的用法相同
/// 
/// # 用法
/// ```
/// print!("Hello, {}!", "world");
/// ```
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

/// Println!宏，用于向主机控制台输出格式化文本并自动添加换行符
/// 接收格式化字符串和可选参数，与标准库println!宏的用法相同
/// 
/// # 用法
/// ```
/// println!("Hello, {}!", "world");
/// ```
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}
