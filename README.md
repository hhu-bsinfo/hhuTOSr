# Aufgabe 1: Ein-/Ausgabe

## Lernziele1. Kennenlernen der Entwicklungsumgebung
2. Einarbeiten in die Programmiersprache C++ 
3. Hardwarenahe Programmierung: CGA-Bildschirm und Tastatur## A1.1: CGA-BildschirmFür Testausgaben und zur Erleichterung der Fehlersuche soll das Betriebssystem zunächst Ausgabefunktionen für den Textbildschirm erhalten. Die Funktionsfähigkeit soll mit Hilfe eines aussagefähigen Testprogramms gezeigt werden, siehe Bildschirmfoto unten.
Dazu soll in `main.cc` in der Einstiegsfunktion `main` das Objekt `kout` für verschieden formatierte Ausgaben genutzt werden. Diese sollen ähnlich wie bei der C++ IO-Streams Bibliothek verwendet werden können. Damit die Ausgabefunktionen überall in hhuTOS genutzt werden kann, ist in der gegebenen Klasse `Gobals`, ein globales `CGA_Stream`-Objekt `kout` bereits definiert.
In folgenden Dateien müssen Quelltexte einfügt werden:
`main.cc`, `devices/CGA.cc` und `user/aufgabe1/TextDemo.cc`.

*Beachten Sie die Kommentare im Quelltext der Vorgabe, sowie die Datei* `CGA.pdf`

### Beispielausgaben

![CGA](https://github.com/mschoett/hhuTOSc/blob/aufgabe-1/img/cga.jpg)


## A1.2: Tastatur
Damit eine Interaktion mit dem Betriebssystem möglich wird benötigen wir einen Tastatur-Treiber. In dieser Aufgabe verwenden wir die Tastatur ohne Interrupts. In main soll die Tastatur in einer Endlos-Schleife abgefragt werden und die Eingaben auf dem CGA-Bildschirm zur Kontrolle ausgegeben werden. 

Beginnen Sie mit der Funktion `key_hit`:
- Prüfen Sie zunächst in einer Schleife, ob ein Datenbyte von der Tastatur vorliegt. Hierzu muss im Control-Port geprüft werden, ob das Bit `OUTB` gesetzt ist.
- Lesen Sie anschließend das Datenbyte über den Daten-Port ein und speichern Sie das gelesene Byte in der gegebenen Variable code.
- Verwenden Sie die vorgegeben Funktion `key_decoded` um jeweils ein gelesenes Datenbyte zu übersetzen. Jedoch müssen Sie zuvor prüfen, ob das Datenbyte nicht von einer PS/2 Maus stammt. Dies wird über das Bit `AUXB` im Control-Register angezeigt. Beim Aufruf von `key_decoded` müssen Sie das das Datenbyte nicht übergeben, dies ist bereits in der Variablen `code` gespeichert.
- Wenn `key_decoded` true zurückgibt wurde eine Taste komplett dekodiert und in der Variablen `gather` gespeichert. Geben Sie in diesem Fall `gather` (Typ `Key`) zurück oder ansonsten `invalid`. 

Danach soll die Funktion `set_repeate_rate` implementiert werden. Zum Schluss können Sie die Funktion `set_led` implementieren (optional).

Namen von benötigten Variablen und Konstanten:
- Control-Port: `ctrl_port`
- Daten-Port: `data_port`
- OUTB: `outb`
- AUXB: `auxb`

Die Befehle für die Implementierung von `set_led` finden Sie in `Keyboard.h`. Warten und prüfen Sie nach dem Absenden eines Befehls die Antwort auf `kbd_reply::ack`. 

In folgenden Dateien müssen Quelltexte einfügt werden: `devices/Keyboard.cc` und `user/aufgabe1/KeyboardDemo.cc`.

*Achtung:
Die Methoden zur Ansteuerung der LEDs und der Tastaturwiederholrate funktionieren nur richtig auf echter Hardware.*

*Beachten Sie die Kommentare im Quelltext der Vorgabe, sowie die Dateien* `Tastatur.pdf` *und* `HinweiseTastatur.pdf`.
