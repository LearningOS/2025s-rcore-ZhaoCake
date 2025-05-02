//ref:: https://github.com/andre-richter/qemu-exit
//! QEMU退出机制实现模块
//! 
//! 本模块基于Andre Richter的qemu-exit库，实现了在RISC-V架构上通过QEMU的sifive_test设备退出模拟器的功能
//! 提供了带有不同退出码的退出方法，便于CI测试和调试
use core::arch::asm;

/// 成功退出的状态码，相当于exit(0)
/// 这个特定的魔术数值会被QEMU识别为成功退出
const EXIT_SUCCESS: u32 = 0x5555; // Equals `exit(0)`. qemu successful exit

/// 失败退出标志位
/// 用于在状态码中标记这是一个失败退出的编码
const EXIT_FAILURE_FLAG: u32 = 0x3333;
/// 失败退出的状态码，相当于exit(1)
/// 表示QEMU应该以错误状态终止
const EXIT_FAILURE: u32 = exit_code_encode(1); // Equals `exit(1)`. qemu failed exit
/// 重置QEMU的状态码
/// 会导致QEMU重新启动而不是退出
const EXIT_RESET: u32 = 0x7777; // qemu reset

/// QEMU退出trait，定义了退出QEMU模拟器的方法接口
/// 
/// 提供了不同退出方式的接口，可以被不同架构的实现类使用
pub trait QEMUExit {
    /// 使用指定的返回码退出QEMU
    /// 
    /// # 参数
    /// * `code` - 退出状态码，用于传达退出的原因
    /// 
    /// # 返回值
    /// * 这个函数不会返回，因为它会导致QEMU退出
    /// 
    /// # 注意
    /// 对于X86架构，退出码会在QEMU内部与0x1进行按位或操作
    fn exit(&self, code: u32) -> !;

    /// 使用EXIT_SUCCESS状态码退出QEMU，代表成功执行
    /// 
    /// # 返回值
    /// * 这个函数不会返回，因为它会导致QEMU退出
    /// 
    /// # 注意
    /// 这在X86架构上不可用
    fn exit_success(&self) -> !;

    /// 使用EXIT_FAILURE状态码退出QEMU，代表执行失败
    /// 
    /// # 返回值
    /// * 这个函数不会返回，因为它会导致QEMU退出
    fn exit_failure(&self) -> !;
}

/// RISCV64架构配置
/// 
/// 包含RISC-V架构下用于退出QEMU的设备地址
pub struct RISCV64 {
    /// sifive_test映射设备的地址
    /// 这是QEMU中模拟的SiFive测试设备的内存映射地址
    addr: u64,
}

/// 使用EXIT_FAILURE_FLAG编码退出码
/// 
/// 将普通的退出码转换为QEMU可以识别的特殊格式
/// 
/// # 参数
/// * `code` - 原始退出码
/// 
/// # 返回值
/// * `u32` - 编码后的退出码
const fn exit_code_encode(code: u32) -> u32 {
    (code << 16) | EXIT_FAILURE_FLAG
}

/// RISCV64结构体的方法实现
impl RISCV64 {
    /// 创建一个新的RISCV64实例
    /// 
    /// # 参数
    /// * `addr` - sifive_test设备的内存映射地址
    /// 
    /// # 返回值
    /// * `Self` - 新创建的RISCV64实例
    pub const fn new(addr: u64) -> Self {
        RISCV64 { addr }
    }
}

/// 为RISCV64实现QEMUExit trait
impl QEMUExit for RISCV64 {
    /// 使用指定的退出码退出QEMU
    /// 
    /// 通过向sifive_test设备写入特定的退出码，触发QEMU退出
    /// 
    /// # 参数
    /// * `code` - 退出状态码
    /// 
    /// # 返回值
    /// * 这个函数不会返回，因为它会导致QEMU退出
    fn exit(&self, code: u32) -> ! {
        // 如果退出码不是特殊值，则需要使用EXIT_FAILURE_FLAG进行编码
        let code_new = match code {
            EXIT_SUCCESS | EXIT_FAILURE | EXIT_RESET => code,
            _ => exit_code_encode(code),
        };

        unsafe {
            // 通过汇编指令向sifive_test设备写入退出码
            asm!(
                "sw {0}, 0({1})",
                in(reg)code_new, in(reg)self.addr
            );

            // 如果QEMU退出尝试失败，进入无限循环
            // 使用wfi指令（等待中断）让CPU休眠以减少功耗
            // 这里不使用panic!是为了避免可能的无限递归
            // （因为panic处理器可能会调用这个函数）
            loop {
                asm!("wfi", options(nomem, nostack));
            }
        }
    }

    /// 使用EXIT_SUCCESS状态码退出QEMU，代表成功执行
    fn exit_success(&self) -> ! {
        self.exit(EXIT_SUCCESS);
    }

    /// 使用EXIT_FAILURE状态码退出QEMU，代表执行失败
    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE);
    }
}

/// SiFive测试设备在QEMU中的内存映射地址
/// 这是QEMU RISC-V virt机器模型中固定的地址
const VIRT_TEST: u64 = 0x100000;

/// QEMU退出处理器的全局实例
/// 使用VIRT_TEST地址初始化的RISCV64结构体
/// 在需要退出QEMU时使用此实例
pub const QEMU_EXIT_HANDLE: RISCV64 = RISCV64::new(VIRT_TEST);
