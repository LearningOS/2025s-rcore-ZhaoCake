//! SBI call wrappers
//! SBI调用封装模块
//! 
//! 本模块提供了对RISC-V SBI（Supervisor Binary Interface）调用的封装
//! SBI是操作系统和机器固件之间的接口，允许操作系统使用更低特权级（M模式）的功能
//! 主要实现了控制台字符输出和系统关闭功能

use core::arch::asm;

/// SBI调用编号：向控制台输出字符
/// 这是SBI规范定义的功能号，用于标识向控制台输出一个字符的操作
const SBI_CONSOLE_PUTCHAR: usize = 1;

/// 通用SBI调用函数
/// 
/// 使用RISC-V的ecall指令进行SBI调用，将参数放入相应寄存器中
/// x16设置为0表示返回值类型（SBI Extension ID）
/// x17保存SBI调用的功能号（Function ID）
/// x10~x12保存调用参数
/// 
/// # 参数
/// * `which` - SBI调用的功能号
/// * `arg0` - 第一个参数
/// * `arg1` - 第二个参数
/// * `arg2` - 第三个参数
/// 
/// # 返回值
/// * `usize` - SBI调用的返回值
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "li x16, 0",
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

/// 通过SBI调用在控制台输出一个字符
/// 
/// 使用SBI_CONSOLE_PUTCHAR功能号调用sbi_call函数，实现向QEMU串口输出字符
/// 
/// # 参数
/// * `c` - 要输出的字符（ASCII码）
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

use crate::board::QEMUExit;
/// 通过SBI调用关闭内核
/// 
/// 使用QEMU的退出机制关闭模拟器，表示内核已经关闭
/// 调用此函数后程序不再返回，因此返回类型为'!'
/// 
/// # 返回值
/// * 永不返回
pub fn shutdown() -> ! {
    crate::board::QEMU_EXIT_HANDLE.exit_failure();
}
