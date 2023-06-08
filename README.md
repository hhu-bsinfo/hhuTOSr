# Aufgabe 6: Semaphore

## Lernziele
1. Verstehen wie Synchronisierung zwischen Threads in Rust funktioniert


## A6.1: Synchronisierung mit Interrupt-Sperre
Erweitern Sie das Testprogramm aus A5.4, indem Sie zwei oder drei Threads starten, welche jeweils einen Zähler an einer festen Position auf dem Bildschirm ausgeben, siehe Bild unten. Zusätzlich soll ein weiterer Thread eine Melodie über den Lautsprecher abspielen.

Sie sollten dann beobachten können, dass die Ausgabe der Zähler nicht wie geplant funktioniert. Überlegen Sie warum dies so ist und synchronisieren Sie die Text-Ausgaben in den Threads durch einen kritischen Abschnitt, den Sie mithilfe von Interrupt-Sperren realisieren.
 
*Achtung: Das Sperren von Interrupts zu Synchronisierungszwecken funktioniert nur auf einem Einkern-BS!*


## A6.2: Mutex
Rust bietet in der Laufzeitumgebung eine Implementierung für Sperren (Mutex). Synchronisieren Sie die Zähler-Threads im Testprogramm aus A6.1 nun mithilfe eines Rust-Mutex. Weitere Informationen dazu finden Sie [hier](https://doc.rust-lang.org/std/sync/struct.Mutex.html).


**Beispielausgabe des Testprogramms**

![Loops](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-6/img/screen.jpg)
