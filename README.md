# Aufgabe 2: Speicherverwaltung und PC-Speaker

## Lernziele1. Verstehen wie eine Speichervwaltung funktioniert und implementiert wird.
2. Hardwarenahe Programmierung: PC-Speaker / Programmable Interval Timer## A2.1: Bump-Allocator
In dieser Aufgabe soll ein sehr einfacher sogenannter Bump-Allocator implementiert werden, um zunächst die Integration in das System zu verstehen sowie die Anbindung an die Programmiersprache. Dieser Allokator kennt lediglich den Heap-Anfang, das Heap-Ende und merkt sich in der Variablen `next` die aktuelle Adresse im Heap, ab welcher der Speicher frei ist. Bei jeder Allokation wird `next` um die gewünschte Anzahl Bytes weitergesetzt, sofern nicht das Heap-Ende erreicht ist, siehe Abbildung.

![Bump-Allocator](https://github.com/mschoett/hhuTOSc/blob/aufgabe-2/img/bump_allocator.jpg)

Die Heapgröße ist fest auf 1 MB eingestellt, im Speicherbereich 3 – 4 MB. Bei einer Speicherfreigabe passiert nichts. Bauen Sie die Vorgabe in Ihr System ein und stellen Sie sicher, dass der Heap möglichst bald in der Einstiegsfunktion des Betriebssystems initialisiert wird.Zur Überprüfung der Implementierung sollen einfache Tests geschrieben werden. Weitere Information hierzu finden sich in den nachfolgenden Hinweisen zur jeweiligen Programmiersprache.

In der Datei `BumpAllocator.cc` soll die Bump-Speicherverwaltung implementiert werden. Die C++Operatoren für `new` und `delete` sind überschrieben und rufen die entsprechenden Funktionen auf.In `Globals.cc/.h` gibt es drei neue globale Variablen: `total_mem` wird in `Allocator.cc` gesetzt und enthält die RAM-Kapazität des Rechners (fest auf 8 MB eingestellt). Zudem gibt es für jeden Allokator ein statisches Objekt, für diese Aufgabe BumpAllocator allocator.
In `boot.asm` wird nun der Label `_ZdlPv` auskommentiert, welcher bisher bei einem `delete` angesprungen wurde. Nun wird `delete` im jeweiligen Allokator aufgerufen. Als Tests sollen in `HeapDemo.cc` Instanzen einer eigenen Klasse sowie Arrays von Objekten und/oder primitiven Datentypen angelegt und freigegeben werden. 

In folgenden Dateien müssen Quelltexte einfügt werden: `user/aufgabe2/HeapDemo.cc` und `kernel/allocator/BumpAllocator.cc`.


## A2.2: Listenbasierter Allokator
In dieser Aufgabe soll ein verbesserter Allokator implementiert werden, welcher freigegeben Speicherblöcke wiederverwenden kann. Hierzu sollen alle freien Blöcke miteinander verkettet werden, siehe Abbildung.

![List-Allocator](https://github.com/mschoett/hhuTOSc/blob/aufgabe-2/img/list_allocator.jpg)

Zu Beginn gibt es nur einen großen freien Speicherblock, der den gesamten freien Speicher umfasst. Im Rahmen der Heap-Initialisierung soll dieser eine freie Block als erster und einziger Eintrag in der verketteten Freispeicherliste gespeichert werden, siehe Abbildung.**Allokation**. Bei der Allokation eines Speicherblocks muss die Freispeicherliste nach einem passenden Block durchsucht werden. Es reicht, wenn immer der erste Block genommen wird, der mindestens die Größe der Allokation erfüllt. Sofern der verbleibende Rest groß genug ist, um die Metadaten eines Listeneintrags zu speichern, so soll dieser abgeschnitten und wieder in die Freispeicherliste eingefügt werden.**Freigabe**. Der freizugebende Block soll in die Freispeicherliste wieder eingehängt werden. Im Prinzip reicht es, wenn er am Anfang der Liste eingefügt wird. Optional kann geprüft werden, ob benachbarte Speicherbereiche auch frei sind und damit verschmolzen werden kann. Dazu muss in der Liste gesucht werden. 
Damit die Freispeicherverwaltung getestet und geprüft werden kann, ist es sinnvoll eine Ausgabe-Funktion zu implementieren, welche die Freispeicherliste komplett auf dem Bildschirm ausgibt. Zudem soll die Test-Anwendung aus Aufgabe 2.1 ausgebaut werden, um auch die Freigabe von Speicherblöcken zu testen.

Die folgenden Hinweise sind Ergänzungen zu denen in Aufgabe A2.1!
In der Datei `LinkedListAllocator.cc` soll die Speicherverwaltung implementiert werden. In`Globals.cc/.h` soll nun LinkedListAllokator allocator verwendet werden.
Es ist zu beachten, dass bei der Allokation neben der angeforderten Speichergröße auch 4 Byte zusätzlich für eine Längenangabe berücksichtigt werden. Dies ist notwendig, damit bei einer Freigabe eines Speicherblocks die Längeninformation verfügbar ist. Es bietet sich an die Länge zu Beginn eines belegten Speicherblocks abzuspeichern und beim Aufruf von `alloc` einen Zeiger direkt nach dem Längenfeld zurückzugeben.
In folgenden Dateien müssen Quelltexte einfügt werden: `user/aufgabe2/HeapDemo.cc` und`kernel/allocator/LinkedListAllocator.cc`.


## A2.3: PC-Lautsprecher
In dieser Aufgabe muss die Methode `PCPSK::delay` implementiert werden. Diese Methode ist für das Abspielen von Tönen notwendig, die eine gegebene Zeitdauer gespielt werden sollen. Da wir bisher keine Interrupts verarbeiten können und auch keine Systemzeit haben bietet es sich an den Zähler 0 des Programmable Interval Timer (PIT) hierfür zu verwenden. Sie können dann in einer Schleife fortlaufend den aktuellen Zählerstand auslesen, der ja mit 1,19 MHz dekrementiert wirdund so näherungsweise die Ausführung, eine gegebene Zeit in Millisekunden, verzögern. Dies ist eine unsaubere Lösung die wir später ersetzen werden.
Hinweis: gute Informationen zum PIT 8254 finden Sie in der Datei `8254.pdf` sowie hier:http://wiki.osdev.org/Programmable_Interval_Timer

In folgenden Dateien müssen Quelltexte einfügt werden: `devices/PCSPK.cc` und`user/aufgabe2/SoundDemo.cc`.


## Beispielausgaben zur Speicherverwaltung
Nachstehend sind einige Screenshots zum Testen der Speicherverwaltung. Sie können sich natürlich selbst Testfunktionen und Testausgaben überlegen. Sollten die Ausgaben über mehrere Seiten gehen bietet es sich an eine Zeitverzögerung mit `pcspk.delay` zu realsieren. Dies ist umständlich und nur als vorübergehende Lösung gedacht; später können wir Tastatureingaben verarbeiten ...

![Heap1](https://github.com/mschoett/hhuTOSc/blob/aufgabe-2/img/heap1.jpg)

![Heap2](https://github.com/mschoett/hhuTOSc/blob/aufgabe-2/img/heap2.jpg)

![Heap3](https://github.com/mschoett/hhuTOSc/blob/aufgabe-2/img/heap3.jpg)

![Heap4](https://github.com/mschoett/hhuTOSc/blob/aufgabe-2/img/heap4.jpg)
