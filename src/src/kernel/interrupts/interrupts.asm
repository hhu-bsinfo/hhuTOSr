;******************************************************************************
;*                                                                            *
;*                  I N T E R R U P T S . A S M                               *
;*                                                                            *
;*----------------------------------------------------------------------------*
;* Beschreibung:    Hier befindet sich alles rund um die low-level Behandlung *
;*                  von Interrupts: IDT, PIC-Initialisierung und Interrupt-   *
;*                  Handler und Aufruf der Interrupt-Behandlung in C.         * 
;*                                                                            *
;* Autor:           Michael Schoettner, 6.7.2022                              *
;******************************************************************************

[GLOBAL _init_interrupts]      ; Funktion exportieren

[EXTERN int_disp]             ; Funktion in C, welche Interrupts behandelt

[SECTION .text]
[BITS 64]


; Exportiere Funktion
_init_interrupts:
   call setup_idt
   call reprogram_pics
   ret




;**********************************************************

;
;   Unterbrechungsbehandlung
;

; Spezifischer Kopf der Unterbrechungsbehandlungsroutinen
%macro wrapper 1
wrapper_%1:
	push   rbp
	mov    rbp, rsp
	push   rax
	mov    al, %1
	jmp    wrapper_body
%endmacro

; ... wird automatisch erzeugt.
%assign i 0
%rep 256
wrapper i
%assign i i+1
%endrep

; Gemeinsamer Rumpf
wrapper_body:
	; Das erwartet der gcc so
	cld
	; Flüchtige Register sichern
	push   rcx
	push   rdx
	push   rdi
	push   rsi
	push   r8
	push   r9
	push   r10
	push   r11

	; Der generierte Wrapper liefert nur 8 Bit
	and    rax, 0xff

	; Nummer der Unterbrechung als Argument übergeben
	mov    rdi, rax
	call   int_disp

	; Flüchtige Register wiederherstellen
	pop    r11
	pop    r10
	pop    r9
	pop    r8
	pop    rsi
	pop    rdi
	pop    rdx
	pop    rcx

	; ... auch die aus dem Wrapper
	pop    rax
	pop    rbp

	; Fertig!
	iretq


;
; Relokation der Eintraege in der IDT und Setzen des IDTR
;

setup_idt:
	mov    rax, wrapper_0

	; Bits 0..15 -> ax, 16..31 -> bx, 32..64 -> edx
	mov    rbx, rax
	mov    rdx, rax
	shr    rdx, 32
	shr    rbx, 16

	mov    r10, idt   ; Zeiger auf das aktuelle Interrupt-Gate
	mov    rcx, 255   ; Zähler
.loop:
	add    [r10+0], ax
	adc    [r10+6], bx
	adc    [r10+8], edx
	add    r10, 16
	dec    rcx
	jge    .loop

	lidt   [idt_descr]
	ret

;
; Neuprogrammierung der PICs (Programmierbare Interrupt-Controller), damit
; alle 15 Hardware-Interrupts nacheinander in der idt liegen.
;

reprogram_pics:
	mov    al, 0x11   ; ICW1: 8086-Modus mit ICW4
	out    0x20, al
	call   delay
	out    0xa0, al
	call   delay
	mov    al, 0x20   ; ICW2 Master: IRQ # Offset (32)
	out    0x21, al
	call   delay
	mov    al, 0x28   ; ICW2 Slave: IRQ # Offset (40)
	out    0xa1, al
	call   delay
	mov    al, 0x04   ; ICW3 Master: Slaves an IRQs
	out    0x21, al
	call   delay
	mov    al, 0x02   ; ICW3 Slave: Verbunden mit IRQ2 des Masters
	out    0xa1, al
	call   delay
	mov    al, 0x03   ; ICW4: 8086-Modus und automatischer EOI
	out    0x21, al
	call   delay
	out    0xa1, al
	call   delay

	mov    al, 0xff   ; Hardware-Interrupts durch PICs
	out    0xa1, al   ; ausmaskieren. Nur der Interrupt 2,
	call   delay      ; der der Kaskadierung der beiden
	mov    al, 0xfb   ; PICs dient, ist erlaubt.
	out    0x21, al

	ret

;
; Kurze Verzögerung für in/out-Befehle
;

delay:
	jmp    .L2
.L2:
	ret



[SECTION .data]

;
; Interrupt Descriptor Table mit 256 Einträgen
;

idt:
%macro idt_entry 1
	dw  (wrapper_%1 - wrapper_0) & 0xffff ; Offset 0 .. 15
	dw  0x0000 | 0x8 * 2 ; Selector zeigt auf den 64-Bit-Codesegment-Deskriptor der GDT
	dw  0x8e00 ; 8 -> interrupt is present, e -> 80386 64-bit interrupt gate
	dw  ((wrapper_%1 - wrapper_0) & 0xffff0000) >> 16 ; Offset 16 .. 31
	dd  ((wrapper_%1 - wrapper_0) & 0xffffffff00000000) >> 32 ; Offset 32..63
	dd  0x00000000 ; Reserviert
%endmacro

%assign i 0
%rep 256
idt_entry i
%assign i i+1
%endrep



idt_descr:
	dw  256*8 - 1    ; 256 Einträge
	dq idt

