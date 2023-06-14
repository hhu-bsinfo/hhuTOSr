# Aufgabe 6: Semaphore

## Lernziele
1. Verstehen wie Synchronisierung zwischen Threads in Rust funktioniert


## A6.1: Synchronisierung mit Interrupt-Sperre
Erweitern Sie das Testprogramm aus A5.4, indem Sie zwei oder zwei Threads starten, welche jeweils einen Zähler an einer festen Position auf dem Bildschirm ausgeben, siehe Bild unten. Zusätzlich soll ein weiterer Thread eine Melodie über den Lautsprecher abspielen.

Sie sollten dann beobachten können, dass die Ausgabe der Zähler nicht wie geplant funktioniert. Überlegen Sie warum dies so ist und synchronisieren Sie die Text-Ausgaben in den Threads durch einen kritischen Abschnitt, den Sie mithilfe von Interrupt-Sperren realisieren.
 
*Achtung: Das Sperren von Interrupts zu Synchronisierungszwecken funktioniert nur auf einem Einkern-BS!*


## A6.2: Mutex
In der Vorgabe finden Sie eine Implementierung für einen Spinlock in `mylib/spinlock.rs`. Synchronisieren Sie die Zähler-Threads im Testprogramm aus A6.1 nun mithilfe dieses Spinlocks. Hierfür muss der Spinlock in beiden Threads schreibend zugegriffen werden. Lesen Sie hierzu folgende Seiten durch: Shared Ownership mit [Shared Ownership](https://doc.rust-lang.org/rust-by-example/std/arc.html) und [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html) 



**Beispielausgabe des Testprogramms**

![Loops](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-6/img/screen.jpg)
