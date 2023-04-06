;******************************************************************************
;*                        B O O T . A S M                                     *
;*----------------------------------------------------------------------------*
;* Die Funktion 'boot_start' ist der Eintrittspunkt des eigentlichen Systems. *
;* Die Umschaltung in den 32-bit 'Protected Mode' ist bereits durch grub      *
;* erfolgt. Es wird alles vorbereitet, damit so schnell wie möglich mit der   * 
;* Ausführung von C++-Code im 64-bit 'Long Mode' begonnen werden kann.        *
;* boot.bin wird an 1 MB geladen und konsumiert mit PageTables 1 MB, sodass   *
;* der C Code oberhalb von 2 MB liegt.                                        *
;*                                                                            *
;* Autor: Michael Schoettner, Uni Duesseldorf, 26.2.2023                      *
;******************************************************************************

;
;   Konstanten
;

; Auskommentieren, um im Grafikmodus zu booten
%define TEXT_MODE 

 
; Lade-Adresse des Kernels, muss mit der Angabe in 'sections' konsistent sein!
KERNEL_START: equ 0x100000


; Stack fuer die main-Funktion
STACKSIZE: equ 65536

; 254 GB maximale RAM-Groesse fuer die Seitentabelle
MAX_MEM: equ 254

; Speicherplatz fuer die Seitentabelle
[GLOBAL pagetable_start]
pagetable_start:  equ 0x103000    ; 1 MB + 12 KB

[GLOBAL pagetable_end]
pagetable_end:  equ 0x200000      ;  = 2 MB

;
;   System
;

; Von uns bereitgestellte Funktionen
[GLOBAL start]
[GLOBAL idt]

; C-Funktion die am Ende des Assembler-Codes aufgerufen werden
[EXTERN startup]


; Vom Compiler bereitgestellte Adressen
[EXTERN ___BSS_START__]
[EXTERN ___BSS_END__]

; In 'sections' definiert
[EXTERN ___KERNEL_DATA_START__]
[EXTERN ___KERNEL_DATA_END__]

; Multiboot constants
MULTIBOOT_HEADER_MAGIC:           equ 0x1BADB002
MULTIBOOT_ARCHITECTURE_I386:      equ 0
MULTIBOOT_HEADER_TAG_OPTIONAL:    equ 1
MULTIBOOT_HEADER_TAG_FRAMEBUFFER: equ 5
MULTIBOOT_HEADER_TAG_END:         equ 0

MULTIBOOT_MEMORY_INFO	equ	1<<1
MULTIBOOT_GRAPHICS_INFO equ 1<<2

MULTIBOOT_HEADER_FLAGS	equ	MULTIBOOT_MEMORY_INFO | MULTIBOOT_GRAPHICS_INFO
MULTIBOOT_HEADER_CHKSUM	equ	-(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

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

[SECTION .text]

;
;   System-Start, Teil 1 (im 32-bit Protected Mode)
;
;   Initialisierung von GDT und Seitentabelle und Wechsel in den 64-bit
;   Long Mode.
;

[BITS 32]

multiboot_header:
	align  4

;
;   Multiboot-Header zum Starten mit GRUB oder QEMU (ohne BIOS)
;
	dd MULTIBOOT_HEADER_MAGIC
	dd MULTIBOOT_HEADER_FLAGS
	dd -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)
	dd multiboot_header   
	dd (___KERNEL_DATA_START__   - KERNEL_START)
	dd (___KERNEL_DATA_END__     - KERNEL_START)
	dd (___BSS_END__        - KERNEL_START)
	dd (startup             - KERNEL_START)
	dd MULTIBOOT_GRAPHICS_MODE
	dd MULTIBOOT_GRAPHICS_WIDTH
	dd MULTIBOOT_GRAPHICS_HEIGHT
	dd MULTIBOOT_GRAPHICS_BPP

;  GRUB Einsprungspunkt
start:
	cld              ; GCC-kompilierter Code erwartet das so
	cli              ; Interrupts ausschalten
	lgdt   [gdt_80]  ; Neue Segmentdeskriptoren setzen

	; Globales Datensegment
	mov    eax, 3 * 0x8
	mov    ds, ax
	mov    es, ax
	mov    fs, ax
	mov    gs, ax

	; Stack festlegen
	mov    ss, ax
	mov    esp, init_stack+STACKSIZE
   
	; Sichere Adresse der Multiboot-Struktur (ist in EBX)
	; da wird den Inhalt erst im 64 Bit Mode wieder herunterholen
	; muessen wir 8 Bytes 'pushen'
    push   0
    push   ebx

	jmp    init_longmode


;
;  Umschalten in den 64 Bit Long-Mode
;
init_longmode:
	; Adresserweiterung (PAE) aktivieren
	mov    eax, cr4
	or     eax, 1 << 5
	mov    cr4, eax

	; Seitentabelle anlegen (Ohne geht es nicht)
	call   setup_paging

	; Long-Mode (fürs erste noch im Compatibility-Mode) aktivieren
	mov    ecx, 0x0C0000080 ; EFER (Extended Feature Enable Register) auswaehlen
	rdmsr
	or     eax, 1 << 8 ; LME (Long Mode Enable)
	wrmsr

	; Paging aktivieren
	mov    eax, cr0
	or     eax, 1 << 31
	mov    cr0, eax

	; Sprung ins 64 Bit-Codesegment -> Long-Mode wird vollständig aktiviert
	jmp    2 * 0x8 : longmode_start


;
;   Anlegen einer (provisorischen) Seitentabelle mit 2 MB Seitengröße, die die
;   ersten MAX_MEM GB direkt auf den physikalischen Speicher abbildet.
;   Dies ist notwendig, da eine funktionierende Seitentabelle für den Long-Mode
;   vorausgesetzt wird. Mehr Speicher darf das System im Moment nicht haben.
;
setup_paging:
	; PML4 (Page Map Level 4 / 1. Stufe)
	mov    eax, pdp
	or     eax, 0xf
	mov    dword [pml4+0], eax
	mov    dword [pml4+4], 0

	; PDPE (Page-Directory-Pointer Entry / 2. Stufe) für aktuell 16GB
	mov    eax, pd
	or     eax, 0x7           ; Adresse der ersten Tabelle (3. Stufe) mit Flags.
	mov    ecx, 0
fill_tables2:
	cmp    ecx, MAX_MEM       ; MAX_MEM Tabellen referenzieren
	je     fill_tables2_done
	mov    dword [pdp + 8*ecx + 0], eax
	mov    dword [pdp + 8*ecx + 4], 0
	add    eax, 0x1000        ; Die Tabellen sind je 4kB groß
	inc    ecx
	ja     fill_tables2
fill_tables2_done:

	; PDE (Page Directory Entry / 3. Stufe)
	mov    eax, 0x0 | 0x87    ; Startadressenbyte 0..3 (=0) + Flags
	mov    ebx, 0             ; Startadressenbyte 4..7 (=0)
	mov    ecx, 0
fill_tables3:
	cmp    ecx, 512*MAX_MEM   ; MAX_MEM Tabellen mit je 512 Einträgen füllen
	je     fill_tables3_done
	mov    dword [pd + 8*ecx + 0], eax ; low bytes
	mov    dword [pd + 8*ecx + 4], ebx ; high bytes
	add    eax, 0x200000      ; 2 MB je Seite
	adc    ebx, 0             ; Overflow? -> Hohen Adressteil inkrementieren
	inc    ecx
	ja     fill_tables3
fill_tables3_done:

	; Basiszeiger auf PML4 setzen
	mov    eax, pml4
	mov    cr3, eax
	ret

;
;   System-Start, Teil 2 (im 64-bit Long-Mode)
;
;   Das BSS-Segment wird gelöscht und die IDT die PICs initialisiert.
;   Anschließend werden die Konstruktoren der globalen C++-Objekte und
;   schließlich main() ausgeführt.
;
longmode_start:
[BITS 64]
    ; zuvor gesicherter Zeiger auf multiboot infos vom Stack holen und
    ; in 'multiboot_info_address' sichern. Durch die Konstruktoren wird 
    ; der Stack manipuliert, daher muessen wir das gleich hier machen
    pop    rax  
    mov    [multiboot_info_address], rax
    
	; BSS löschen
	mov    rdi, ___BSS_START__
clear_bss:
	mov    byte [rdi], 0
	inc    rdi
	cmp    rdi, ___BSS_END__
	jne    clear_bss

;	fninit         ; FPU aktivieren
	
    mov    rdi, [multiboot_info_address] ; 1. Parameter wird in rdi uebergeben
	call   startup ; multiboot infos auslesen und 'main' aufrufen
	
	cli            ; Hier sollten wir nicht hinkommen
	hlt



;
; Kurze Verzögerung für in/out-Befehle
;
delay:
	jmp    .L2
.L2:
	ret


;
; Funktionen für den C++ Compiler. Diese Label müssen für den Linker
; definiert sein; da bei OOStuBS keine Freigabe des Speichers erfolgt, können
; die Funktionen aber leer sein.
;
__cxa_pure_virtual: ; "virtual" Methode ohne Implementierung aufgerufen
;_ZdlPv:             ; void operator delete(void*)
;_ZdlPvj:            ; void operator delete(void*, unsigned int) fuer g++ 6.x
;_ZdlPvm:            ; void operator delete(void*, unsigned long) fuer g++ 6.x
	ret


[SECTION .data]

;
; Segment-Deskriptoren
;
gdt:
	dw  0,0,0,0   ; NULL-Deskriptor

	; 32-Bit-Codesegment-Deskriptor
	dw  0xFFFF    ; 4Gb - (0x100000*0x1000 = 4Gb)
	dw  0x0000    ; base address=0
	dw  0x9A00    ; code read/exec
	dw  0x00CF    ; granularity=4096, 386 (+5th nibble of limit)

	; 64-Bit-Codesegment-Deskriptor
	dw  0xFFFF    ; 4Gb - (0x100000*0x1000 = 4Gb)
	dw  0x0000    ; base address=0
	dw  0x9A00    ; code read/exec
	dw  0x00AF    ; granularity=4096, 386 (+5th nibble of limit), Long-Mode

	; Datensegment-Deskriptor
	dw  0xFFFF    ; 4Gb - (0x100000*0x1000 = 4Gb)
	dw  0x0000    ; base address=0
	dw  0x9200    ; data read/write
	dw  0x00CF    ; granularity=4096, 386 (+5th nibble of limit)

gdt_80:
	dw  4*8 - 1   ; GDT Limit=24, 4 GDT Eintraege - 1
	dq  gdt       ; Adresse der GDT

multiboot_info_address:
	dq  0

[SECTION .bss]

global init_stack:data (init_stack.end - init_stack)
init_stack:
	resb STACKSIZE
.end:


;
; Speicher fuer Page-Tables
;
[SECTION .global_pagetable]

[GLOBAL pml4]
[GLOBAL pdp]
[GLOBAL pd]

pml4:
    times 4096 db 0
	alignb 4096

pd:
    times MAX_MEM*4096 db 0
	alignb 4096

pdp:
    times MAX_MEM*8 db 0    ; 254*8 = 2032

