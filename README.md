# Aufgabe 3: Interrupts

## Lernziele

1. Funktionsweise der Interrupt Descriptor Table (IDT) verstehen
2. Funktionsweise des Interrupt-Controllers verstehen
2. Behandlung von Interrupts implementieren, am Beispiel der Tastatur

## A3.1: Interrupt Descriptor Table (IDT)
In dieser Aufgabe soll die Interrupt Descriptor Table (IDT) erstellt und geladen werden.

In der Datei `kernel/interrupts/idt.rs` ist bereits ein Großteil des Codes zu Erstellung einer IDT vorgegeben.
Die IDT hat 256 Einträge, wobei jeder Eintrag auf eine Funktion verweist, die angesprungen werden soll, wenn der entsprechende Interrupt auftritt. In hhuTOS verweist jeder Eintrag auf die Funktion `int_disp()` aus `kernel/interrupts/intdispatcher.rs`, welche die weitere Verarbeitung der Interrupts vornimmt.
Zusätzlich hat jeder Eintag noch einige Flags, die korrekt gesetzt werden müssen (`IdtEntry::options`).

Implementieren Sie zunächst die Funktion `IdtEntry::new()`, welche einen neuen IDT-Eintrag erzeugen soll. Der Parameter `offset` repräsentiert dabei die Adresse der anzuspringenden Funktion und muss innerhalb des Eintrags auf drei einzelne Teile aufgeteilt werden (`IdtEntry::offset_low`, `IdtEntry::offset_mid`, `IdtEntry::offset_high`). Außerdem soll jeder Eintrag immer die Optionen `Present`, `DPL = 0` und `64-Bit Interrupt Gate` gesetzt haben. Details zum Aufbau eines IDT-Eintrags finden Sie im [OSDev Wiki](https://wiki.osdev.org/Interrupt_Descriptor_Table#Structure_on_x86-64).

Laden Sie die IDT in `startup.rs` mit `idt::get_idt().load()`. Nun sollte bei jedem Interrupt die Funktion `int_disp()` aufgerufen werden. Um das zu testen, fügen Sie eine Ausgabe in die (noch leere) Funktion `int_disp()` ein. Hierfür soll `kprintln!()` und nicht `println!()` verwendet werden. Zudem sollte `kprintln!()` nicht in Anwendungscode genutzt werden. Hintergrund ist, dass die `kprintln()!` und `println!()` Makros intern einen Mutex verwenden, welcher eventuell während der Interrupt-Verarbeitung gerade durch die Anwendung gesperrt ist. In diesem Fall würde eine Verklemmung auftreten.

Um manuell einen Interrupt auszulösen können Sie die x86-Instruktion `int` in `startup.rs` verwenden: `unsafe { asm!("int 100"); }` sollte nun `int_disp()` mit dem Parameter `vector = 100` anspringen. 

In folgenden Dateien muss Code implementiert werden: `kernel/interrupts/idt.rs`, `startup.rs`.

## A3.2: Programmable Interrupt Controller (PIC)
In dieser Aufgabe sollen Hardware Interrupts aktiviert und anhand der Tastatur getestet werden.

Zunächst müssen die leeren Funktionen in `pic.rs` implementiert werden. 

Anschliessend soll in `keyboard.rs` die Funktion `plugin` programmiert werden. Hier muss der IRQ der Tastatur am `PIC` mit `allow()` freigeschaltet werden. Die ISR `keyboard::trigger()` kann vorerst leer bleiben. Auch das Registrieren der ISR der Tastatur folgt später.

In `startup.rs` muss die `init()` Funktion des PIC aufgerufen, sowie die ISR der Tastatur mit `keyboard::plugin()` registriert werden. Anschliessend müssen die Interrupts an der CPU mit `cpu::enable_int()` zugelassen werden.

Wenn nun das System startet sollte bei jedem Drücken und Loslassen einer Taste eine Textmeldung von `int_disp()` zu sehen sein. Dies funktioniert allerdings nur einige wenige male (oder sogar nur ein einziges mal). Wenn die Zeichen nicht vom Tastaturcontroller abgeholt werden, läuft der Tastaturpuffer irgendwann voll. Sobald der Puffer voll ist, sendet der Tastaturcontroller keine Interrupts mehr.

In folgenden Dateien muss Code implementiert werden: `kernel/interrupts/pic.rs`,
`devices/keyboard.rs`, `startup.rs` und `kernel/interrupts/int_dispatcher.rs`.

*Allgemeine Hinweise:*
- *Während der Behandlung einer Unterbrechung braucht man sich um unerwünschte Interrupts nicht zu sorgen. Der Prozessor schaltet diese nämlich automatisch aus, wenn er mit der Behandlung beginnt, und lässt sie erst wieder zu, wenn die Unterbrechungsbehandlung beendet wird. Zudem nutzen wir nur einen Prozessor-Kern.*
- *Die Interrupt-Verarbeitung kann nur funktionieren, wenn hhuTOS auch läuft. Sobald hhuTOS die main-Funktion verlässt, ist das Verhalten bei Auftreten eines Interrupts undefiniert. Ein Betriebssystem sollte eben nicht plötzlich enden :-)*


**Beispielausgaben in `int_disp()`**:
```
Welcome to hhuTOS!
Initializing heap allocator
Initializing PIC
Initializing interrupts
int_disp: Interrupt 100!
Initializing keyboard
Enabling interrupts
Boot sequence finished
int_disp: Interrupt 33!
```

## A3.3: Weiterleitung von Interrupts an die Geräte-Treiber
In dieser Aufgabe soll eine Infrastruktur geschaffen werden, um Interrupts, welche in `int_disp()` (siehe Aufgabe A3.2) entgegengenommen werden, an eine zuvor registierte Interrupt-Service-Routine (ISR) in einem Treiber weiterzuleiten.

Ein Treiber muss hierfür eine ISR implementieren und registrieren. Die Schnittstelle der ISR besteht „nur“ aus der `trigger()` Funktion. Zu beachten ist, dass der Interrupt-Dispatcher mit Vektor-Nummern arbeitet und nicht IRQ-Nummern wie der PIC.

Zur Verwaltung der ISR verwendet das Modul `intdispatcher` die dynamische Datenstruktur `Vec`, welche mit 256 Options, die den Wert `None` beinhalten, gefüllt wird. Dies erlaubt es in `register()` eine ISR eines Treibers (Schnittstelle definiert in `isr`) an einem gegebenen Index zu speichern. Leider geht dies in Rust nicht mit einem Array statischer Größe. 

Die Funktion `report()` soll von `int_disp()` aufgerufen werden, um die Funktion `trigger()` einer registrierten ISR-Funktion aufrufen, sofern vorhanden. Falls keine ISR registriert wurde, also `None` eingetragen ist, so soll eine Fehlermeldung ausgegeben und das System gestoppt werden. Entfernen Sie nun unbedingt den manuellen Test-Interrupt in `startup.rs`, da es sonst zu genau diesem Fall kommt.

Um `report()` aufzurufen muss der Mutex um `INT_VECTORS` gelocked werden. Normalerweise ist es keine gute Idee, während eines Interrupts ein Lock zu holen, da es zu einer Verklemmung kommt falls das Lock bereits vergeben ist. In diesem Fall würde der Interrupt Handler nie zurückkehren und das Betriebssystem hängen bleiben (Gleiche Problematik wie bei `println!()`/`kprintln!()`). Um das zu verhindern, sollten alle Interrupt Handler registriert werden, bevor die Interrupts mit `cpu::enable_int()` eingeschaltet werden.

Im Modul `keyboard` muss die Funktion `plugin()` erweitert werden und eine Referenz auf ein Funktionsobjekt `KeyboardISR` mithilfe von `register()` (im Modul `intdispatcher`) registrieren. Die für die Tastatur notwendige Vektor-Nummer ist in `intdispatcher::InterruptVector` definiert. 

Des Weiteren soll eine Text-Ausgabe in die Funktion `trigger()` eingebaut werden, um zu prüfen, ob die Tastaturinterrupts hier ankommen. Auch hier soll für Textausgaben `kprintln!()` verwendet werden.

In folgenden Dateien muss Code implementiert werden: `devices/keyboard.rs`, `kernel/interrupts/intdispatcher.rs` und `startup.rs`.

**Beispielausgaben in `keyboard::trigger()`**:
```
Welcome to hhuTOS!
Initializing heap allocator
Initializing PIC
Initializing interrupts
Initializing keyboard
Enabling interrupts
Boot sequence finished
int_disp: Interrupt 33!
keyboard::trigger called!
```

## A3.4: Tastaturabfrage per Interrupt
Nun soll die Funktion `trigger()` in `keyboard` implementiert werden. Bei jedem Interrupt soll `key_hit_irq()` aufgerufen, ein Byte eingelesen werden und geprüft werden, ob ein Zeichen erfolgreich dekodiert wurde. Wenn dies der Fall ist, so soll der ASCII-Code des Zeichens in die neue globale Variable `KEYBOARD_BUFFER` eingereiht werden. Dabei handelt es sich um eine Queue, auf welche Anwendungen später mit `keyboard::get_key_buffer()` zugreifen und Tasten auslesen können. In `library/input.rs` sind zwei Beispielfunktionen die `keyboard::get_key_buffer()` verwenden. Für die Queue verwenden wir die Crate [nolock](https://lib.rs/crates/nolock), welche Datenstrukturen zur Verfügung stellt, die ohne Locks auskommen und trotzdem konkurriernde Zugriffe unterstützen. Solche Strukturen eignen sich perfekt für die Interruptverarbeitung, da wir ja innerhalb eines Interrupt Handlers normalerweise keine Locks holen dürfen.

In `trigger()` muss die globale Variable `KEYBOARD` gelocked werden, damit `key_hit_irq()` mit einer mutable self Referenz aufgerufen werden kann. Das ist in Ordnung, da die gesamte Tastenverarbeitung in `trigger()` stattfindet und `KEYBOARD` an keiner anderen Stelle mehr gelocked wird.

Bauen Sie nun alle bisherigen Demos so um, dass sie nicht mehr `key_hit()` verwenden um auf einen Tastendruck zu warten, sondern stattdessen Tasten aus dem `KEYBOARD_BUFFER` abholen. Die Funktion `key_hit()` kann nun nicht mehr verwendet werden und sollte gelöscht werden.

*Hinweise:*
- *In `key_hit_irq()` sollte zumindest ein Byte eingelesen werden, da ansonsten keine weitere Interrupts von der Tastatur durchkommen.*
- *Die PS/2-Maus hängt ebenfalls am Keyboard-Controller, verwendet aber IRQ12. Da wir keinen Handler für IRQ12 haben, kann es sein, dass wenn IRQ1 auftritt noch Daten von der Maus abzuholen sind. Dies können Sie anhand des `AUXB`-Bits im Statusregister erkennen.*
- *Ferner tritt unter Qemu manchmal direkt ein IRQ1 nach dem Start auf, ohne eine Tastatureingabe. Das ist auf echter Hardware nicht der Fall. Daher unter Qemu bitte ignorieren.*
