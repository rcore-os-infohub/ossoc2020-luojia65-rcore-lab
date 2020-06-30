MEMORY {
    /* 起始地址 */
	DRAM : ORIGIN = 0x80000000, LENGTH = 128M
}

/* 通常情况下都是从REGION_TEXT区域的起始位置运行的，但qemu的opensbi规定了入口位置，就把程序放在这里 */
PROVIDE(_stext = 0x80200000);
/* 如果要扩栈就改这个数 */
PROVIDE(_hart_stack_size = 128K);
/* 加核心的时候同时需要改这个数 */
PROVIDE(_max_hart_id = 4);
/* 加frame需要改这个数 */
PROVIDE(_frame_size = 16384 * 4K);

REGION_ALIAS("REGION_TEXT", DRAM);
REGION_ALIAS("REGION_RODATA", DRAM);
REGION_ALIAS("REGION_DATA", DRAM);
REGION_ALIAS("REGION_BSS", DRAM);
REGION_ALIAS("REGION_STACK", DRAM);
REGION_ALIAS("REGION_FRAME", DRAM);
