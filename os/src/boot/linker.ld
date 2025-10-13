ENTRY(start)

PHDRS
{
    text PT_LOAD;
    rodata PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

SECTIONS {
    . = 1M;   /* load at address 1MB */

    ___KERNEL_DATA_START__ = .;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        *(.multiboot_header)
    } :text

    .text :
    {
        *(.text*)
    } :text

    .rodata :
    {
        *(.rodata*)
    } :rodata

    .data :
    {
        *(.data*)
    } :data

   .bss : 
    {
      ___BSS_START__ = .;
      *(".bss.*")
      ___BSS_END__ = .;
    } :bss

    ___KERNEL_DATA_END__ = .;

}
