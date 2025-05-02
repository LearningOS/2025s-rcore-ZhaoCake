//! The panic handler
//! panic处理模块
//!
//! 本模块实现了Rust的panic处理机制，当内核发生严重错误时，会调用这里的处理函数
//! 在裸机环境下，我们需要自己实现panic_handler以处理崩溃情况

use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
/// panic handler
/// panic处理函数
/// 
/// 当内核出现严重错误（如断言失败、数组越界等）时，Rust会调用此函数
/// 该函数会打印错误位置和消息，然后关闭系统
/// 
/// # 参数
/// * `info` - panic的详细信息，包含文件路径、行号和错误消息
/// 
/// # 返回值
/// * 永不返回，因为系统将关闭
fn panic(info: &PanicInfo) -> ! {
    // 判断错误信息中是否包含位置信息（文件名和行号）
    if let Some(location) = info.location() {
        // 如果有位置信息，打印包含文件名和行号的错误信息
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),        // 发生panic的源文件路径
            location.line(),        // 发生panic的源文件行号
            info.message().unwrap() // panic的错误消息
        );
    } else {
        // 如果没有位置信息，只打印错误消息
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    // 关闭系统，不再继续执行
    shutdown()
}
