# Aufgabe 5: Preemptives Multithreading

## Lernziele
1. Tieferes Verständnis von präemptiven Multitasking
2. CPU-Entzug mithilfe des PIT
3. Synchronisierung des Schedulers gegenüber dem PIT-Interrupt


## A5.1: Programmable Interval Timer (PIT)
Der PIT wird ab sofort verwendet, um eine Systemzeit sowie ein erzwungenes Umschalten zwischen Threads zu realisieren. Die Systemzeit wird in der Variable `systime` (in `pit.rs`) gespeichert und diese wird bei jedem Interrupt für den PIT inkrementiert. Verwenden Sie hierfür im PIT den Channel 0 und Modus 3 und laden Sie den Zähler mit einem passenden Wert, sodass der PIT alle 10ms ein Interrupt ausgelöst. Jeder Interrupt verursacht also eine Inkrementierung und entspricht einem Tick (10ms). Somit zeigt `systime` an, wie viele Ticks seit dem Beginn der Zeiterfassung
vergangen sind. 

Geben Sie im Interrupt-Handler des PIT den Fortschritt der Systemzeit an einer festen Stelle aus, jedoch nicht bei jedem Tick, sondern nur alle 100 Ticks. Verwenden Sie hierfür beispielsweise die rechte obere Ecke und folgende Zeichen |, /, -, \, wobei das Zeichen bei jeder Ausgabe wechselt. Dadurch, dass ein Zeichenausgabe nur alle 100 Ticks erfolgt, ändert sich das Zeichen ungefähr jede Sekunde. 

Testen Sie die Klasse `PIT` indem Sie die ISR für den PIT in `startup` registrieren und die Ausgabe der Systemzeit überprüfen. Die Systemzeit (in Ticks) wird in `pit.rs` realisert. 

In folgenden Dateien muss Code implementiert werden: `devices/pit.rs` und `startup.rs`.

## A5.2: Umbau des Treibers für den PC-Lautsprecher
Die `delay` Funktion im Treiber für den PC-Lautsprecher hat bisher den PIT direkt programmiert und soll nun die Systemzeit aus A5.1 nutzen. Hierzu soll eine Busy-Waiting-Schleife verwendet werden, welche stoppt, wenn die `systime` entsprechend der gewünschten Verzögerung fortgeschritten ist. Busy-Waiting ist nicht schön, aber durch das präemptive Multithreading akzeptabel. 

Testen Sie den Umbau mit einer der Melodien.

In folgender Datei muss Code implementiert werden: `devices/pcspk.rs`.


## A5.3 Threadumschaltung mithilfe des PIT
In der Vorgabe ist neu die Funktion `preempt`in `scheduler.rs`. Diese Methode soll bei jedem Tick aus der ISR vom PIT aufgerufen werden und eine erzwungene Threadumschaltung durchführen. Somit entspricht eine Zeitscheibe einem Tick.  

Zusätzliche müssen die Methoden des Schedulers mithilfe von Interrupt-Sperren gegenüber dem PIT synchronisiert werden. Ansonsten kann es sein, dass die Ready-Queue kaputt geht, wenn in einem ungünstigen Augenblick `preempt`aufgerufen wird.

Zudem muss sichergestellt werden, dass der Scheduler fertig initialisiert ist, bevor das erste Mal `preempt`versucht umzuschalten. Dies ist in der Vorgabe bereits realsierit. Der Idel-Thread setzt die Variable `initialized` auf `true`, sobald er erstmalig losläuft.

In folgender Datei muss Code implementiert werden: `kernel/threads/scheduler.rs`.


## A5.4: Testanwendung mit Multithreading
Testen Sie das präemptive Multitasking indem Sie eine kleine Demo-Anwendung schreiben in der ein Zähler-Thread läuft, welcher einen Zähler inkrementiert und an einer festen Position auf dem Bildschirm ausgibt. Zusätzlich soll noch ein zweiter Thread erzeugt werden der eine Melodie abspielt. Neben diesen beiden Threads soll zusätzlich der Fortschritt der Systemzeit im Interrupt ausgegeben werden, siehe nachstehende Abbildung.


**Beispielausgab des Testprogramms**

![MTHR](https://github.com/hhu-bsinfo/hhuTOSr/blob/aufgabe-5/img/mthr.jpg)
