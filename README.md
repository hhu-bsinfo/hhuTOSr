# Aufgabe 3: Interrupts

## Lernziele

1. Funktionsweise des Interrupt-Controllers verstehen
2. Behandlung von Interrupts implementieren, am Beispiel der Tastatur
3. Kritische Abschnitte (Synchronisierung) verstehen und einsetzen
## A3.1: Programmable Interrupt Controller (PIC)
In dieser Aufgabe soll die Interrupt-Verarbeitung aktiviert und anhand der Tastatur geprüft werden. Zunächst muss der PIC programmiert werden, um den Interrupt für die Tastatur zuzulassen. Zudem muss der Prozessor Interrupts vom PIC annehmen, was mithilfe der Funktion `cpu.enable_int` gemacht wird. Wenn dies funktioniert, müsste automatisch bei jedem Drücken und Loslassen einer Taste die Funktion `int_disp`, der Interrupt-Dispatcher in C aufgerufen werden, da die Interrupt-Vektortabelle im Assembler-Code (siehe Vorgabe) entsprechend initialisiert wurde.
Mit einer Ausgabe in `int_disp` kann dieser Schritt leicht getestet werden, zumindest einige Male. Wenn die Zeichen nicht vom Tastaturcontroller abgeholt werden, läuft der Tastaturpuffer irgendwann voll. Sobald der Puffer voll ist, sendet der Tastaturcontroller keine Interrupts mehr. Deshalb kann es durchaus passieren, dass zunächst nur für ein oder zwei Tastendrücke Interrupts auftreten.

In der Datei `intdispatcher.rs` soll in der Funktion `int_disp` eine Textausgabe eingefügt werden. Hierfür sollen nicht die Makros `print!` und `println!` von Rust verwendet werden, sondern direkt auf das Modul cga zugegriffen werden. Hintergrund ist, dass die print-Makros einen Mutex verwenden, welcher eventuell während der Interrupt-Verarbeitung gerade durch die Anwendung gesperrt ist. In diesem Fall würde eine Verklemmung auftreten.
In `keyboard.rs` soll in der vorgegebenen Funktion plugin die ISR der Tastatur in dem Modul`intdispatcher` registriert (mit der Funktion `assign`) sowie im Modul `pic` die Interrupts von der Tastatur freigeschaltet werden. Die Funktion `trigger` ist im Modul `Keyboard` bereits als leere Funktion schon vorhanden.
In `startup.rs` soll die Funktion `plugin` von der Tastatur aufgerufen werden und danach die Interruptverarbeitung durch die CPU, mithilfe des Moduls `cpu`, freigeschaltet werden.Die IDT wird durch den in `startup.rs` vorhandenen Aufruf `interrupts::init` eingerichtet. Dadurch wird bei jedem Interrupt die Funktion `int_disp` in 'kernel/interrupts/mod.rs`  aufgerufen.In folgenden Dateien muss Code implementiert werden: `kernel/interrupts/pic.rs`,`devices/keyboard.rs`, `startup.rs` und `kernel/interrupts/int_dispatcher.rs`.

*Allgemeine Hinweise:*- *Während der Behandlung einer Unterbrechung braucht man sich um unerwünschte Interrupts nicht zu sorgen. Der Prozessor schaltet diese nämlich automatisch aus, wenn er mit der Behandlung beginnt, und lässt sie erst wieder zu, wenn die Unterbrechungsbehandlung beendet wird. Zudem nutzen wir nur einen Prozessor-Kern.*- *Die Interrupt-Verarbeitung kann nur funktionieren, wenn HHUos auch läuft. Sobald HHUos die main-Funktion verlässt, ist das Verhalten bei Auftreten eines Interrupts undefiniert. Ein Betriebssystem sollte eben nicht plötzlich enden :-)*


**Beispielausgaben in** `int_disp`

![IRQ1](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-3/img/irq1.jpg)

## A3.2: Weiterleitung von Interrupts an die Geräte-Treiber
In dieser Aufgabe soll eine Infrastruktur geschaffen werden, um Interrupts, welche in `int_disp` (siehe Aufgabe A3.1) entgegengenommen werden, an eine Interrupt-Service-Routine (ISR) in einem Treiber weiterzuleiten.
Ein Treiber muss hierfür eine ISR implementieren und registrieren. Die Schnittstelle der ISR besteht „nur“ aus der `trigger`-Funktion. Zu beachten ist, dass der Interrupt-Dispatcher mit Vektor-Nummern arbeitet und nicht IRQ-Nummern wie der PIC.

Zur Verwaltung der ISR verwendet das Modul `intdispatcher` die dynamische Datenstruktur `Vec`,welche mit 256 Default-ISRs (Funktionsobjekte) initialisiert wird. Dies erlaubt es in `assign` eine ISR eines Treibers (Schnittstelle definiert in `isr`) an einem gegebenen Index zu speichern. Leider geht dies in Rust nicht mit einem Array statischer Größe. 

Die Funktion `report` soll von `int_disp` gerufen werden, um die Funktion trigger einer registrierten isr-Funktion aufrufen, sofern vorhanden. Falls keine ISR registriert wurde, also nur der Default-Handler eingetragen ist, so soll eine Fehlermeldung ausgegeben werden und das System gestoppt werden.
Im Modul `keyboard` soll die Funktion `plugin` eine Referenz auf ein Funktionsobjekt `KeyboardISR`, mithilfe von `assign` (im Modul `intdispatcher`) registrieren. Die für die Tastatur notwendige Vektor-Nummer ist in `intdispatcher` definiert. Zudem muss der IRQ der Tastatur (definiert in `pic.rs`) im PIC mithilfe von `allow` freigeschaltet werden.
Des Weiteren soll eine Text-Ausgabe in die Funktion `trigger` eingebaut werden, um zu prüfen, ob die Tastaturinterrupts hier ankommen.
Für Textausgaben in einem Interrupt (in `int_disp` und registrierten ISRs) sollten nicht die üblichen Makros `print!` und `println!` von Rust verwendet werden, sondern direkt auf das Modul `cga` zugegriffen werden. Der Hintergrund wurde in A.3.1 beschrieben.

In folgenden Dateien muss Code implementiert werden: `kernel/interrupts/pic.rs`,`devices/keyboard.rs`, `startup.rs`, und `kernel/interrupts/intdispatcher.rs`.

**Beispielausgaben in** `Keyboard::trigger`

![IRQ2](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-3/img/irq2.jpg)


## A3.3: Tastaturabfrage per Interrupt
Nun soll die Funktion `trigger` in Keyboard implementiert werden und mithilfe von `key_hit` bei jedem Tastendruck das entsprechende Zeichen an einer festen Position auf dem Bildschirm dargestellt werden. Die Ausgabe des Zeichens soll direkt in `trigger` erfolgen.
Der ASCII-Code der zuletzt gedrückten Taste soll zudem in der neuen Variable `lastkey` gespeichert werden, welche später von Anwendungen ausgelesen werden kann. In `lib/Input.cc` sind zwei Beispielfunktionen welche `lastkey` verwenden, beispielsweise warten bis der Benutzer die Taste Return gedrückt hat.*Hinweise:*- *Die PS/2-Maus hängt ebenfalls am Keyboard-Controller, verwendet aber IRQ12. Da wir keinen Handler für IRQ12 haben, kann es sein, dass wenn IRQ1 auftritt noch Daten von der Maus abzuholen sind. Dies können Sie anhand des* `AUXB`*-Bits im Statusregister erkennen.*- *Ferner tritt unter Qemu manchmal direkt ein IRQ1 nach dem Start auf, ohne eine Tastatureingabe. Das ist auf echter Hardware nicht der Fall. Daher unter Qemu bitte ignorieren.*

**Beispielausgaben in** `Keyboard::trigger` **an einer festen Position**

![IRQ3](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-3/img/irq3.jpg)


## A3.4: kritische Abschnitte
Hier soll ein Testprogramm in geschrieben werden. Dieses soll in einer Endlosschleife an einer festen Position Ausgaben machen. Zum Beispiel zeilenweise die Zahlen 0-9.
Es sollte nun möglich sein, durch das Drücken von Tasten die Ausgabe "durcheinander" bringen zu können. Was passiert hier? Wie kann dies vermieden werden?*Tipp: Für die Synchronisierung / Implementierung eines kritischen Abschnitts gibt es nützliche Funktionen in der Klasse* `CPU`.

**Beispielausgaben "Durcheinander" ohne Synchronisierung**

![IRQ4](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-3/img/irq4.jpg)


