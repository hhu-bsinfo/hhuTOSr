#*****************************************************************************
#* ACHTUNG:         Ein falsch angegebenes Laufwerk kann dazu fuehren, dass  *
#*                  Daten auf dem spezifizierten Laufwerk verloren gehen!    *
#*                  Nicht mit root-Rechten ausfuehren!                       *
#*                                                                           *
#* Autor:           Michael Schoettner, HHU, 12.03.2023                      *
#*****************************************************************************/
BOOTDRIVE = /dev/sdb

TARGET := hhu_tosr
RUST_OBJ := target/$(TARGET)/debug/lib$(TARGET).a

ASM = nasm
ASMOBJFORMAT = elf64
ASMFLAGS := -w+error=label-redef-late
OBJDIR = build

SYSTEM := build/kernel-$(TARGET).bin
LINKER_SCRIPT := src/boot/linker.ld
GRUB_CFG := src/boot/grub.cfg
ISO := build/$(TARGET).iso

# -------------------------------------------------------------------------
# Namen der Unterverzeichnisse mit den Assembler-Quelltexten
VPATH = $(sort $(dir $(ASM_SOURCES)))

# --------------------------------------------------------------------------
# Liste der Assembler-Quelltexte/-Objektdateien
ASM_SOURCES = $(shell find ./src -name "*.asm")
ASM_OBJECTS = $(patsubst %.asm,_%.o, $(notdir $(ASM_SOURCES)))
OBJPRE = $(addprefix $(OBJDIR)/,$(ASM_OBJECTS))

VERBOSE = @

.PHONY: all clean run iso

all: $(ISO)

clean:
	@rm -r build
	@cargo clean
	
# --------------------------------------------------------------------------
# Regeln zur Erzeugung der Assembler-Objektdateien
$(OBJDIR)/_%.o : %.asm
	@echo "ASM		$@"
	@if test \( ! \( -d $(@D) \) \) ;then mkdir -p $(@D);fi
	$(VERBOSE) $(ASM) -f $(ASMOBJFORMAT) $(ASMFLAGS) -o $@ $<

# --------------------------------------------------------------------------
# Regeln zum Compilieren der Rust-Dateien 
rust_objs:
	@RUST_TARGET_PATH=$(shell pwd) xargo build -Z build-std=std,panic_abort --target $(TARGET)

# --------------------------------------------------------------------------
# System binden
$(SYSTEM): rust_objs $(OBJPRE) $(LINKER_SCRIPT)
	@x86_64-elf-ld -n -T $(LINKER_SCRIPT) -o $(SYSTEM) $(OBJPRE) $(RUST_OBJ)

# --------------------------------------------------------------------------
# 'bootdisk' erzeugt zunaechst das System, falls das noch nicht geschehen ist.
# Danach wird das Image auf das Block-Device 'BOOTDRIVE' geschrieben
bootdisk:
	sudo dd bs=1K if=$(ISO) of=$(BOOTDRIVE) conv=fdatasync

# --------------------------------------------------------------------------
# 'qemu' ruft den qemu-Emulator mit dem System auf.
qemu: $(ISO)
	@qemu-system-x86_64 -cdrom $(ISO) -audiodev coreaudio,id=snd0 -machine pcspk-audiodev=snd0

# --------------------------------------------------------------------------
# ISO erstellen
$(ISO): $(SYSTEM) $(GRUB_CFG)
	@mkdir -p build/isofiles/boot/grub
	@cp $(SYSTEM) build/isofiles/boot/kernel.bin
	@cp $(GRUB_CFG) build/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) build/isofiles 2> /dev/null
	@rm -r build/isofiles

# --------------------------------------------------------------------------
# 'qemu-gdb' ruft den qemu-Emulator mit aktiviertem GDB-Stub mit dem System
# auf, sodass es per GDB inspiziert werden kann.
qemu-gdb:
	$(VERBOSE) echo "break startup" > /tmp/gdbcommands.$(shell id -u)
	$(VERBOSE) echo "target remote 127.0.0.1:1234" >> /tmp/gdbcommands.$(shell id -u)
	$(VERBOSE) echo "continue" >> /tmp/gdbcommands.$(shell id -u)
	qemu-system-x86_64 -cdrom $(ISO) -soundhw pcspk -s -S 

# --------------------------------------------------------------------------
# 'gdb' startet den GDB-Debugger und verbindet sich mit dem GDB-Stub des vorher
# gestarteten Qemu.
gdb:
	gdb -x /tmp/gdbcommands.$(shell id -u) $(SYSTEM)
	
help:
	@/bin/clear
	@/bin/echo -e "\n" \
		"\e[1mMAKEFILE fuer hhuTOSr\e[0m\n" \
		"---------------------\n\n" \
		"Durch Aufruf von '\e[4mmake\e[0m' wird das Betriebssystem compiliert und gebunden.\n" \
		"Danach wird die Datei 'system.iso' im Verzeichnis 'build' erzeugt.\n"
	@/bin/echo -e " Emulation\n" \
		"	\e[3mqemu\e[0m        Startet das Betriebssystem in QEMU\n" \
		"	            Achtung: Manchmal ist das Verhalten des Beitriebssystems\n" \
		"	            in Qemu nicht exakt wie auf echter Hardware!\n\n" \
		"	\e[3mqemu-gdb\e[0m    Startet Qemu mit GDB-Unterstuetzung und wartet, dass GDB\n" \
		"	            sich verbindet. Dies geht mit dem Aufruf 'make qemu-gdb'.\n"
	@/bin/echo -e " Bootfaehiges Medium\n" \
		"	\e[3mbootdisk\e[0m    Schreibt die zuvor erzeugte 'system.iso' Datei auf den\n" \
		"                   konfigurierten USB-Stick. Hierzu muss im Makefile die\n" \
        "                   Variable \e[4mBOOTDRIVE\e[0m auf die richtige Geraetedatei zeigen.\n" \
        "                   Fuer den Zugriff auf die Geraetedatei sind root-Rechte \n" \
        "                   notwendig.\n" \
        "                   \e[1mAchtung:\e[0m Alle Daten auf dem Stick werden geloescht.\n" \
        "                   Sollte die Gerätedatei falsch sein, werden u.U. Daten \n" \
        "                   auf der Festplatte geloescht!\n\n"
