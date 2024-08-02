MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 2048K
  CCMRAM (rwx) : ORIGIN = 0x10000000, LENGTH = 64K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 192K
  SDRAM (rw): ORIGIN = 0xC0000000, LENGTH = 32M
}

SECTIONS
{
    .sdram (NOLOAD) : ALIGN(4)
    {
        *(.sdram .sdram.*);
        . = ALIGN(4);
    } > SDRAM
}


/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
