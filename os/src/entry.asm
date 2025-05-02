.section .text.entry
    .globl _start
# 操作系统入口点
# 这是RISC-V架构下操作系统的汇编入口点，由引导加载程序跳转到此处开始执行
# _start符号被设置为全局可见，便于链接器找到入口点
_start:
    # 设置栈指针到栈顶
    # la指令将boot_stack_top的地址加载到sp寄存器中
    # sp是栈指针寄存器，指向当前栈帧的顶部
    la sp, boot_stack_top
    
    # 调用Rust入口函数rust_main
    # call指令会将返回地址保存到ra寄存器中，然后跳转到rust_main函数
    # rust_main函数定义在main.rs中，是操作系统的Rust入口点
    call rust_main

# 内核栈段
# .bss.stack段用于存放内核栈，由链接器脚本指定在BSS区域
.section .bss.stack
    .globl boot_stack_lower_bound
# 内核栈的下界地址
# 标记栈的起始位置，用于栈溢出检测和调试信息输出
boot_stack_lower_bound:
    # 分配64KB的栈空间（4096字节 * 16）
    # .space伪指令用于在当前位置分配指定字节数的空间，初始值为0
    .space 4096 * 16
    .globl boot_stack_top
# 内核栈的顶部地址
# 栈是向下增长的，因此初始栈指针指向栈的顶部
boot_stack_top: