# Aufgabe 3: Interrupts

## Lernziele

1. Funktionsweise des Interrupt-Controllers verstehen
2. Behandlung von Interrupts implementieren, am Beispiel der Tastatur
3. Kritische Abschnitte (Synchronisierung) verstehen und einsetzen
## A3.1: Programmable Interrupt Controller (PIC)
In dieser Aufgabe soll die Interrupt-Verarbeitung aktiviert und anhand der Tastatur geprüft werden.

Zunächst müssen die leeren Funktionen in dem Modul `PIC` implementiert werden. In der Funktion `int_disp` (in `intdispatcher.rs`) soll vorerst eine Textausgabe eingefügt werden, welche ausgibt, dass ein Interrupt aufgetreten ist und welche Vektor-Nummer dieser hat. Hierfür sollen nicht die Makros `print!` und `println!` von Rust verwendet werden, sondern direkt auf das Modul `cga` zugegriffen werden. Hintergrund ist, dass die `print!`-Makros intern einen Mutex verwenden, welcher eventuell während der Interrupt-Verarbeitung gerade durch die Anwendung gesperrt ist. In diesem Fall würde eine Verklemmung auftreten.

Anschliessend soll die Funktion `plugin` in `keyboard.rs` programmiert werden. Hier muss der Interrupt der Tastatur am `PIC` mit `pic::allow` freigeschaltet werden. Die Funktion `pic::trigger` kann vorerst leer bleiben.

In `startup` muss die ISR der Tastatur mit `keyboard::plugin()` registriert werden und danach muessen die Interrupts an der CPU mit `cpu::enable_int()` zugelassen werden. In der Vorgabe wird der PIC bereits durch Aufruf von `interrupts::init()` initialisiert.


Wenn nun das System startet sollte bei jedem Drücken und Loslassen einer Taste eine Textmeldung von `int_disp` zu sehen sein. Dies funktioniert allerdings nur einige wenige Male. Wenn die Zeichen nicht vom Tastaturcontroller abgeholt werden, läuft der Tastaturpuffer irgendwann voll. Sobald der Puffer voll ist, sendet der Tastaturcontroller keine Interrupts mehr.

Die IDT wird durch den in `startup.rs` vorhandenen Aufruf `interrupts::init` eingerichtet. Dadurch wird bei jedem Interrupt die Funktion `int_disp` in `kernel/interrupts/mod.rs`  aufgerufen.

In folgenden Dateien muss Code implementiert werden: `kernel/interrupts/pic.rs`,`devices/keyboard.rs`, `startup.rs` und `kernel/interrupts/int_dispatcher.rs`.

*Allgemeine Hinweise:*- *Während der Behandlung einer Unterbrechung braucht man sich um unerwünschte Interrupts nicht zu sorgen. Der Prozessor schaltet diese nämlich automatisch aus, wenn er mit der Behandlung beginnt, und lässt sie erst wieder zu, wenn die Unterbrechungsbehandlung beendet wird. Zudem nutzen wir nur einen Prozessor-Kern.*- *Die Interrupt-Verarbeitung kann nur funktionieren, wenn HHUos auch läuft. Sobald HHUos die main-Funktion verlässt, ist das Verhalten bei Auftreten eines Interrupts undefiniert. Ein Betriebssystem sollte eben nicht plötzlich enden :-)*


**Beispielausgaben in** `int_disp`

![IRQ1](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-3/img/irq1.jpg)

## A3.2: Weiterleitung von Interrupts an die Geräte-Treiber
In dieser Aufgabe soll eine Infrastruktur geschaffen werden, um Interrupts, welche in `int_disp` (siehe Aufgabe A3.1) entgegengenommen werden, an eine Interrupt-Service-Routine (ISR) in einem Treiber weiterzuleiten.
Ein Treiber muss hierfür eine ISR implementieren und registrieren. Die Schnittstelle der ISR besteht „nur“ aus der `trigger`-Funktion. Zu beachten ist, dass der Interrupt-Dispatcher mit Vektor-Nummern arbeitet und nicht IRQ-Nummern wie der PIC.

Zur Verwaltung der ISR verwendet das Modul `intdispatcher` die dynamische Datenstruktur `Vec`,welche mit 256 Default-ISRs (Funktionsobjekte) initialisiert wird. Dies erlaubt es in `assign` eine ISR eines Treibers (Schnittstelle definiert in `isr`) an einem gegebenen Index zu speichern. Leider geht dies in Rust nicht mit einem Array statischer Größe. 

Die Funktion `report` soll von `int_disp` gerufen werden, um die Funktion trigger einer registrierten isr-Funktion aufrufen, sofern vorhanden. Falls keine ISR registriert wurde, also nur der Default-Handler eingetragen ist, so soll eine Fehlermeldung ausgegeben werden und das System gestoppt werden.
Im Modul `keyboard` soll muss die Funktion `plugin` erweitert werden und soll eine Referenz auf ein Funktionsobjekt `KeyboardISR`, mithilfe von `assign` (im Modul `intdispatcher`) registrieren. Die für die Tastatur notwendige Vektor-Nummer ist in `intdispatcher` definiert. 
Des Weiteren soll eine Text-Ausgabe in die Funktion `trigger` eingebaut werden, um zu prüfen, ob die Tastaturinterrupts hier ankommen. Auch hier soll für Textausgaben direkt auf das Modul `cga` zugegriffen werden (Begründung siehe oben). 

In folgenden Dateien muss Code implementiert werden: `kernel/interrupts/pic.rs`,`devices/keyboard.rs`, `startup.rs`, und `kernel/interrupts/intdispatcher.rs`.

**Beispielausgaben in** `Keyboard::trigger`

![IRQ2](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-3/img/irq2.jpg)


## A3.3: Tastaturabfrage per Interrupt
Nun soll die Funktion `trigger` in `keyboard` implementiert werden. Bei jedem Interrupt soll `key_hit` aufgerufen und geprüft werden, ob ein Zeichen erfolgreich dekodiert wurde. Wenn dies der Fall ist, so soll der ASCII-Code des Zeichens in der neuen Variable `lastkey` gespeichert werden, welche später von Anwendungen ausgelesen werden kann. In `mylib/inut.rs` sind zwei Beispielfunktionen welche `lastkey` verwenden, beispielsweise warten bis der Benutzer die Taste Return gedrückt hat.

Falls `key_hit` in Aufgabe 1 so realisiert wurde, dass in einer Endlos-Schleife Daten von der Tastatur eingelesen werden, bis ein Zeichen erfolgreich dekodiert wurde, so muss diese Endlos-Schleife entfernt werden. Es sollen nun bei jedem Interrupt nur so viele Bytes von der Tastatur eingelesen werden, wie unmittelbar vorhanden sind. Wir wollen nicht den Interrupt blockieren durch aktives Warten auf weitere Bytes.
*Hinweise:*- *Die PS/2-Maus hängt ebenfalls am Keyboard-Controller, verwendet aber IRQ12. Da wir keinen Handler für IRQ12 haben, kann es sein, dass wenn IRQ1 auftritt noch Daten von der Maus abzuholen sind. Dies können Sie anhand des* `AUXB`*-Bits im Statusregister erkennen.*- *Ferner tritt unter Qemu manchmal direkt ein IRQ1 nach dem Start auf, ohne eine Tastatureingabe. Das ist auf echter Hardware nicht der Fall. Daher unter Qemu bitte ignorieren.*

**Beispielausgaben in** `Keyboard::trigger` **an einer festen Position**

![IRQ3](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-3/img/irq3.jpg)


## A3.4: kritische Abschnitte
Es soll ein Testprogramm in geschrieben werden, welches in einer Endlosschleife an einer festen Position Text-Ausgaben mach, zeilenweise die Zahlen 0 - 9.

Es sollte nun möglich sein, durch das Drücken von Tasten die Ausgabe "durcheinander" bringen zu können. Was passiert hier? Wie kann dies vermieden werden?


*Tipp: Für die Synchronisierung / Implementierung eines kritischen Abschnitts gibt es nützliche Funktionen in der Klasse* `CPU`.

**Beispielausgaben "Durcheinander" ohne Synchronisierung**

![IRQ4](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-3/img/irq4.jpg)


