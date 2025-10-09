;
; File: boot.asm
;
; Description: Entry point for the operating system. The bootloader (GRUB)
;              has already switched to 32-bit protected mode. The function
;              '_start' is called by the bootloader as it hands off control
;              to the OS. This code sets up a stack, the Global Descriptor Table (GDT),
;              initializes paging with an identity (1:1) mapping and switches
;              to 64-bit long mode before entering Rust code by calling 'startup'.
;
; Author: Michael Schoettner, Heinrich Heine University Duesseldorf, 30.10.2023
;

; Comment out to boot in graphical mode
%define TEXT_MODE

; Load address of the kernel (1 MiB - Must be consistent with the linker script)
KERNEL_START: equ 0x100000

; Stack size
STACKSIZE: equ 65536

; Maximum supported RAM size (used to set up initial page tables)
MAX_MEM: equ 254

; Memory for page tables
[GLOBAL _pagetable_start]
_pagetable_start:  equ 0x103000 ; 1 MiB + 12 KB

[GLOBAL _pagetable_end]
_pagetable_end:  equ 0x200000 ;  = 2 MB

; Multiboot constants
MULTIBOOT_HEADER_MAGIC:           equ 0x1BADB002
MULTIBOOT_ARCHITECTURE_I386:      equ 0
MULTIBOOT_HEADER_TAG_OPTIONAL:    equ 1
MULTIBOOT_HEADER_TAG_FRAMEBUFFER: equ 5
MULTIBOOT_HEADER_TAG_END:         equ 0

MULTIBOOT_MEMORY_INFO equ 1 << 1
MULTIBOOT_GRAPHICS_INFO equ 1 << 2

MULTIBOOT_HEADER_FLAGS equ MULTIBOOT_MEMORY_INFO | MULTIBOOT_GRAPHICS_INFO
MULTIBOOT_HEADER_CHKSUM equ -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

%ifdef TEXT_MODE
    MULTIBOOT_GRAPHICS_MODE    equ 1
    MULTIBOOT_GRAPHICS_WIDTH   equ 80
    MULTIBOOT_GRAPHICS_HEIGHT  equ 25
    MULTIBOOT_GRAPHICS_BPP     equ 0
%else
    MULTIBOOT_GRAPHICS_MODE   equ 0
    MULTIBOOT_GRAPHICS_WIDTH  equ 800
    MULTIBOOT_GRAPHICS_HEIGHT equ 600
    MULTIBOOT_GRAPHICS_BPP    equ 32
%endif

; Exported symbols (for linking with Rust code)
[GLOBAL _get_tss_address] ; Function to get the address of the TSS
[GLOBAL _tss_set_rsp0] ; Function to set the kernel stack pointer (rsp0) in the TSS
[EXTERN startup] ; Entry point in Rust code

; External symbols from the linker script
[EXTERN ___BSS_START__]
[EXTERN ___BSS_END__]
[EXTERN ___KERNEL_DATA_START__]
[EXTERN ___KERNEL_DATA_END__]

[SECTION .text]

;
; Boot Part 1 (in 32-bit Protected Mode)
;
; Initialize GDT and page tables, then switch to 64-bit Long Mode.
;

[BITS 32]

; Multiboot header (contains information for the bootloader)
_multiboot_header:
    align  4
    dd MULTIBOOT_HEADER_MAGIC
    dd MULTIBOOT_HEADER_FLAGS
    dd -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)
    dd _multiboot_header
    dd (___KERNEL_DATA_START__   - KERNEL_START)
    dd (___KERNEL_DATA_END__     - KERNEL_START)
    dd (___BSS_END__             - KERNEL_START)
    dd (startup                  - KERNEL_START)
    dd MULTIBOOT_GRAPHICS_MODE
    dd MULTIBOOT_GRAPHICS_WIDTH
    dd MULTIBOOT_GRAPHICS_HEIGHT
    dd MULTIBOOT_GRAPHICS_BPP

; Entry point called by the bootloader
_start:
    cli ; Disable interrupts
    lgdt [_gdt_descriptor] ; Load the GDT

    ; Set data segment registers
    mov eax, (3 << 3) ; Third entry in GDT = Data segment (Index starts at third bit, so we shift left by 3)
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Set up stack
    mov ss, ax ; Stack segment = Data segment
    mov esp, _init_stack + STACKSIZE ; Let ESP point to the top of the stack

    ; Store multiboot info address (passed by bootloader in EBX)
    mov [_multiboot_addr], ebx

    ; Long jump sets code segment register (index 1 = 32-bit code segment)
    jmp (1 << 3) : _init_longmode ; First entry in GDT = Code segment (Index starts at third bit, so we shift left by 3)

;
;  Switch to 64-bit long mode
;
_init_longmode:
    ; Enable address extension (PAE) in CR4
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; Setup page tables for identity mapping
    call   _setup_paging

    ; Activate long mode (still in 32-bit compatibility mode)
    mov ecx, 0x0C0000080 ; Choose EFER (Extended Feature Enable Register) as model specific register
    rdmsr ; Read current value of EFER
    or eax, 1 << 8 ; LME (Long Mode Enable)
    wrmsr ; Write back to EFER

    ; Activate paging in CR0
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ; Long jump to 64-bit code segment -> Leave 32-bit compatibility mode
    jmp (2 << 3) : _longmode_start ; Second entry in GDT = 64-bit code segment (Index starts at third bit, so we shift left by 3)

;
; Set up a provisional page table with 2 MB page size that maps the first 'MAX_MEM' GiB
; directly to physical memory. This is necessary because a functioning page table
; is required for long mode. The system must not have more memory at the moment.
;
_setup_paging:
    ; PML4 (Page Map Level 4 / First Level)
    mov eax, _pdp
    or eax, 0xf
    mov dword [_pml4 + 0], eax
    mov dword [_pml4 + 4], 0

    ; PDPE (Page-Directory-Pointer Entry / Second level)
    mov eax, _pd
    or eax, 0x7 ; Address of first table with flags (present, rw, user)
    mov ecx, 0
_fill_tables2:
    cmp ecx, MAX_MEM ; Reference 'MAX_MEM' tables with 512 entries each
    je _fill_tables2_done
    mov dword [_pdp + 8 * ecx + 0], eax
    mov dword [_pdp + 8 * ecx + 4], 0
    add eax, 0x1000 ; Each table is 4 KiB in size
    inc ecx
    ja _fill_tables2
_fill_tables2_done:

    ; PDE (Page Directory Entry / Third level)
    mov eax, 0x0 | 0x87 ; Start address byte 0..3 (=0) + flags (present, rw, user, page size = 2 MB)
    mov ebx, 0 ; Start address byte 4..7 (=0)
    mov ecx, 0
_fill_tables3:
    cmp    ecx, 512 * MAX_MEM ; Fill 'MAX_MEM' tables with 512 entries each
    je     _fill_tables3_done
    mov    dword [_pd + 8 * ecx + 0], eax ; Low bytes
    mov    dword [_pd + 8 * ecx + 4], ebx ; High bytes
    add    eax, 0x200000 ; 2 MB je Seite
    adc    ebx, 0 ; Overflow? -> Increment high bytes
    inc    ecx
    ja     _fill_tables3
_fill_tables3_done:

    ; Load PML4 (First level page table) into CR3
    mov    eax, _pml4
    mov    cr3, eax
    ret

;
; Boot part 2 (in 64-bit Long Mode)
;
; The BSS segment is cleared and the TSS is initialized.
; Finally, 'startup' in Rust code is called.
;

[BITS 64]

_longmode_start:

    ; Clear BSS
    mov    rdi, ___BSS_START__
_clear_bss:
    mov    byte [rdi], 0
    inc    rdi
    cmp    rdi, ___BSS_END__
    jne    _clear_bss

   ; Set TSS base address in GDT entry
   call _tss_set_base_address

   ; Set kernel stack in TSS (rsp0)
    mov rdi, _init_stack.end
   call _tss_set_rsp0

   ; Load TSS register with the TSS descriptor

   ;
   ; Hier muss Code eingefuegt werden
   ;

   ; Call startup with multiboot info address as parameter
    xor rax, rax
    mov dword eax, _multiboot_addr
    mov rdi, [rax] ; First parameter is passed in RDI
    call startup ; Call Rust code

    ; We should never return here
    cli ; Disable interrupts
    hlt ; Stop execution

; Set TSS base address in GDT entry
_tss_set_base_address:
;
; Hier muss Code eingefuegt werden
;
ret


; Set kernel stack (rsp0) in TSS
; First Parameter -> RDI = Pointer to kernel stack
_tss_set_rsp0:
;
; Hier muss Code eingefuegt werden
;
ret

; Get address of the TSS
_get_tss_address:
    mov rax, _tss
    ret

[SECTION .data]

; Global Descriptor Table (GDT) with 4 entries:
;
; 0: NULL descriptor (always required)
; 1: 32-Bit kernel code segment (only needed for booting)
; 2: 64-Bit kernel code segment
; 3: 64-Bit kernel data segment
_gdt:
    ; NULL descriptor (always required)
    dw  0, 0, 0, 0

    ; 32-Bit kernel code segment (only needed for booting)
    dw  0xFFFF    ; Limit [00:15] = 4 GiB (0x100000 * 0x1000 = 4 GiB)
    dw  0x0000    ; Base  [00:15] = 0
    dw  0x9A00    ; Base  [16:23] = 0, code read/exec, DPL = 0, present
    dw  0x00CF    ; Limit [16:19], granularity = 4096, 386, base [24:31]

    ; 64-Bit kernel code segment
    dw  0xFFFF    ; Limit [00:15] = 4 GiB (0x100000 * 0x1000 = 4 GiB)
    dw  0x0000    ; Base  [00:15] = 0
    dw  0x9A00    ; Base  [16:23] = 0, code read/exec, DPL = 0, present
    dw  0x00AF    ; Limit [16:19], granularity = 4096, 386, Long-Mode, base [24:31]

    ; 64-Bit kernel data segment
    dw  0xFFFF    ; Limit [00:15] = 4 GiB (0x100000 * 0x1000 = 4 GiB)
    dw  0x0000    ; Base  [00:15] = 0
    dw  0x9200    ; Base  [16:23] = 0, data read/write, DPL = 0, present
    dw  0x00CF    ; Limit [16:19], granularity = 4096, 386, base [24:31]

; GDT descriptor for LGDT instruction
_gdt_descriptor:
    align 16
    dw  4 * 8 - 1 ; GDT limit = 31 (4 entries of 8 bytes each)
    dq  _gdt ; Address of GDT

; Address of the multiboot information structure is stored here during boot
_multiboot_addr:
    dq 0

; Task state segment (TSS)
; 104 Byte without IO bitmap
; (See https://stackoverflow.com/questions/54876039/creating-a-proper-task-state-segment-tss-structure-with-and-without-an-io-bitm)
_tss:
    times 100 db 0
    dw 0
    dw 0x68 ; IO bitmap offset (no IO bitmap, so set to size of TSS 0x68 = 104)

[SECTION .bss]

; Kernel stack
global _init_stack:data (_init_stack.end - _init_stack)
_init_stack:
    resb STACKSIZE
.end:

; Page tables
[SECTION .global_pagetable]

[GLOBAL _pml4]
[GLOBAL _pdp]
[GLOBAL _pd]

_pml4:
    align 4096
    times 4096 db 0

_pd:
    align 4096
    times MAX_MEM * 4096 db 0

_pdp:
    times MAX_MEM * 8 db 0    ; 254 * 8 = 2032
