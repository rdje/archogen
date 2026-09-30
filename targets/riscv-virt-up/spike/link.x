/* The spike's memory layout: everything in the RAM `/memory@80000000` names, where `-bios none` starts the image. */
OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY
{
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
  .text : { KEEP(*(.text.init)) *(.text .text.*) } > RAM
  .rodata : { *(.rodata .rodata.* .srodata .srodata.*) } > RAM
  .data : { *(.data .data.* .sdata .sdata.*) } > RAM
  .bss (NOLOAD) : ALIGN(8)
  {
    __bss_start = .;
    *(.bss .bss.* .sbss .sbss.*)
    . = ALIGN(8);
    __bss_end = .;
  } > RAM
  . = ALIGN(16);
  . = . + 16K;
  __stack_top = .;
}
