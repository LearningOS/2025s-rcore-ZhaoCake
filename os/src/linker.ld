/* 
 * 内核链接脚本
 * 本文件定义了操作系统内核的内存布局
 * 指定了各个段（section）的位置和对齐方式
 */

/* 指定生成RISC-V架构的可执行文件 */
OUTPUT_ARCH(riscv)

/* 指定程序入口点为_start符号（定义在entry.asm中） */
ENTRY(_start)

/* 定义内核加载的基地址为0x80200000 */
/* 这是QEMU的RISC-V virt机器模型中可用的内存起始地址 */
BASE_ADDRESS = 0x80200000;

/* 定义内存段布局 */
SECTIONS
{
    /* 将内核放置在基地址处 */
    . = BASE_ADDRESS;
    /* 定义内核起始符号skernel */
    skernel = .;

    /* 代码段（.text）放在最前面 */
    /* 标记代码段起始位置为stext */
    stext = .;
    .text : {
        /* 首先放置入口点代码，确保其位于内存的开始位置 */
        *(.text.entry)
        /* 然后放置所有其他的代码段 */
        *(.text .text.*)
    }

    /* 将当前位置按4K（页大小）对齐 */
    . = ALIGN(4K);
    /* 标记代码段结束位置为etext */
    etext = .;
    
    /* 只读数据段（.rodata）紧随代码段之后 */
    /* 标记只读数据段起始位置为srodata */
    srodata = .;
    .rodata : {
        /* 放置所有的只读数据段 */
        *(.rodata .rodata.*)
        /* 放置所有的小型只读数据段 */
        *(.srodata .srodata.*)
    }

    /* 将当前位置按4K对齐 */
    . = ALIGN(4K);
    /* 标记只读数据段结束位置为erodata */
    erodata = .;
    
    /* 数据段（.data）紧随只读数据段之后 */
    /* 标记数据段起始位置为sdata */
    sdata = .;
    .data : {
        /* 放置所有的数据段 */
        *(.data .data.*)
        /* 放置所有的小型数据段 */
        *(.sdata .sdata.*)
    }

    /* 将当前位置按4K对齐 */
    . = ALIGN(4K);
    /* 标记数据段结束位置为edata */
    edata = .;
    
    /* BSS段（未初始化的数据）放在最后 */
    .bss : {
        /* 首先放置内核栈 */
        *(.bss.stack)
        /* 标记BSS段起始位置为sbss */
        sbss = .;
        /* 放置所有的BSS段 */
        *(.bss .bss.*)
        /* 放置所有的小型BSS段 */
        *(.sbss .sbss.*)
    }

    /* 将当前位置按4K对齐 */
    . = ALIGN(4K);
    /* 标记BSS段结束位置为ebss */
    ebss = .;
    /* 标记内核结束位置为ekernel */
    ekernel = .;

    /* 丢弃不需要的段 */
    /* .eh_frame段包含异常处理信息，在裸机环境下不需要 */
    /DISCARD/ : {
        *(.eh_frame)
    }
}