//! The main module and entrypoint
//! 主模块和入口点
//!
//! 操作系统和应用程序的启动都从这个模块开始。内核代码从`entry.asm`开始执行，
//! 随后调用[`rust_main()`]初始化各种功能，其中包括[`clear_bss()`]（清除BSS段）。
//! （查看源代码获取详细信息）
//!
//! 然后我们调用[`println!`]显示"Hello, world!"
//!
//! 该模块是操作系统的核心入口，负责进行初始化和输出欢迎信息

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

// 将板级支持包文件路径设置为QEMU相关实现
// 使用path属性指定模块的实际文件路径，便于支持不同的硬件平台
#[path = "boards/qemu.rs"]
mod board;

// 包含汇编入口文件
// global_asm!宏将指定的汇编代码包含到当前环境中
global_asm!(include_str!("entry.asm"));

/// clear BSS segment
/// 清除BSS段函数
///
/// BSS(Block Started by Symbol)段存储未初始化的全局变量和静态变量
/// 操作系统启动时需要将这些变量初始化为0，本函数完成这一任务
/// 通过遍历BSS段的每个字节并写入0来完成初始化
pub fn clear_bss() {
    extern "C" {
        fn sbss();  // BSS段开始地址，由链接器脚本定义
        fn ebss();  // BSS段结束地址，由链接器脚本定义
    }
    // 将sbss到ebss范围内的每个字节都设置为0
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// the rust entry-point of os
/// 操作系统的Rust入口点
///
/// 这是从汇编入口点跳转到的第一个Rust函数
/// 负责初始化操作系统的各个组件并输出调试信息
/// 该函数永不返回，因此返回类型为!'
#[no_mangle]  // 不对函数名进行名称修饰，确保汇编代码能够找到此函数
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();                // 代码段开始地址
        fn etext();                // 代码段结束地址
        fn srodata();              // 只读数据段开始地址
        fn erodata();              // 只读数据段结束地址
        fn sdata();                // 数据段开始地址
        fn edata();                // 数据段结束地址
        fn sbss();                 // BSS段开始地址
        fn ebss();                 // BSS段结束地址
        fn boot_stack_lower_bound(); // 启动栈的下界地址
        fn boot_stack_top();       // 启动栈的顶部地址
    }
    clear_bss();           // 清除BSS段，初始化为0
    logging::init();       // 初始化日志系统，设置日志级别和输出格式
    println!("[kernel] Hello, world!");  // 输出欢迎消息，表示内核已启动
    
    // 下面的代码使用不同级别的日志输出内存布局信息
    // 每个日志输出显示不同内存段的起始和结束地址
    // 使用trace!、debug!、info!、warn!和error!来表示不同的日志级别
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    use crate::board::QEMUExit;
    crate::board::QEMU_EXIT_HANDLE.exit_success(); // CI自动测试成功
                                                   //crate::board::QEMU_EXIT_HANDLE.exit_failure(); // CI自动测试失败
}
