//! Global logger
//! 全局日志系统
//! 
//! 本模块实现了一个简单的日志系统，用于在内核中记录不同级别的日志信息
//! 支持不同级别的日志（ERROR、WARN、INFO、DEBUG、TRACE）并使用不同颜色进行显示
//! 日志级别可以通过环境变量LOG配置

use log::{Level, LevelFilter, Log, Metadata, Record};

/// a simple logger
/// 一个简单的日志记录器
/// 
/// 实现了log crate的Log trait，提供基本的日志功能
/// 根据日志级别显示不同颜色，增强可读性和区分度
struct SimpleLogger;

impl Log for SimpleLogger {
    /// 判断是否启用指定元数据的日志
    /// 
    /// # 参数
    /// * `_metadata` - 日志元数据，包含日志级别等信息
    /// 
    /// # 返回值
    /// * 当前实现总是返回true，表示启用所有日志
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    
    /// 记录日志的核心方法
    /// 
    /// 根据日志级别选择不同的颜色进行输出，增强可读性
    /// 
    /// # 参数
    /// * `record` - 日志记录，包含级别、消息等信息
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        // 根据日志级别选择不同的ANSI颜色码
        let color = match record.level() {
            Level::Error => 31, // 红色 - 用于错误信息
            Level::Warn => 93,  // 亮黄色 - 用于警告信息
            Level::Info => 34,  // 蓝色 - 用于一般信息
            Level::Debug => 32, // 绿色 - 用于调试信息
            Level::Trace => 90, // 亮黑色 - 用于跟踪信息
        };
        // 使用ANSI转义序列设置颜色并输出日志
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
    
    /// 刷新日志
    /// 
    /// 当前实现为空，因为println!已经自动刷新
    fn flush(&self) {}
}

/// initiate logger
/// 初始化日志系统
/// 
/// 设置全局日志记录器并根据环境变量配置日志级别
/// 环境变量LOG可以设置为ERROR、WARN、INFO、DEBUG、TRACE等值
/// 如果未设置环境变量，则默认关闭日志
pub fn init() {
    // 创建静态日志记录器实例
    static LOGGER: SimpleLogger = SimpleLogger;
    // 设置全局日志记录器
    log::set_logger(&LOGGER).unwrap();
    // 根据环境变量设置日志级别
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,  // 仅显示错误
        Some("WARN") => LevelFilter::Warn,    // 显示错误和警告
        Some("INFO") => LevelFilter::Info,    // 显示错误、警告和信息
        Some("DEBUG") => LevelFilter::Debug,  // 显示错误、警告、信息和调试
        Some("TRACE") => LevelFilter::Trace,  // 显示所有级别的日志
        _ => LevelFilter::Off,                // 默认关闭日志
    });
}
