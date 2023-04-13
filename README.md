# Aufgabe 4: Koroutinen und Threads

## Lernziele
1. Auffrischen der Assemblerkenntnisse
2. Verständnis der Abläufe bei einem Koroutinen-Wechsel
3. Unterschied zwischen Threads und Koroutinen
3. Verstehen wie ein Scheduler funktioniert


## A4.1: Koroutinen
In dieser Aufgabe soll die Umschaltung zwischen Koroutinen in Assembler programmiert werden. Koroutinen sind eine Vorstufe zu Threads die später (siehe unten) zusätzlich eingeführt werden. 

Sehen Sie sich zunächst die Inhalte der neuen Dateien in der Vorgabe im Ordner `kernel/corouts` an und implementieren Sie die beiden Assemblerfunktionen `coroutine_start` und `coroutine_switch` in `coroutine.asm`. Der Zustand (alle Register) einer Koroutine soll auf dem Stack gesichert werden. Das `rflags`-Register kann nicht direkt per move-Befehl zugegriffen werden, sondern nur mithilfe der Instruktionen `popf` und `pushf`. 

Der Zeiger auf den letzten Stack-Eintrag soll in der Instanzvariablen `context` in der Klasse `Coroutine` gespeichert werden.

Ergänzen Sie anschließend die leeren Methoden in `Coroutine.cc`. Die Verkettung der Koroutinen erfolgt mit der Klasse `lib/Chain`.

Schreiben Sie für Ihre Koroutinen-Implementierung folgendes Testprogramm. Im Verzeichnis
`user` der Vorgabe finden Sie hierfür Dateien. Es sollen drei Koroutinen erzeugt und miteinander
verkettet werden. Jede Koroutine soll einen Zähler hochzählen und an einer festen Position auf dem Bildschirm ausgeben und dann auf die nächste Koroutine umschalten. Durch die Verkettung werden die drei Koroutinen dann reihum abwechselnd ausgeführt, wodurch die Zähler scheinbar nebenläufig vorangetrieben werden, siehe nachstehende Abbildung.

In folgenden Dateien muss Code implementiert werden: `kernel/corouts/Coroutine.asm`, `kernel/corouts/Coroutine.cc`, `user/aufgabe4/CoroutineDemo.cc`, `user/aufgabe4/CoroutineLoop.cc`

Hinweis: Schauen Sie sich vor dem Programmieren der Assemblerfunktionen nochmals die Aufrufkonvention für die Parameterübergabe an.


**Beispielausgaben der Koroutinen**

![KOR1](https://github.com/mschoett/hhuTOSc/blob/aufgabe-4/img/corouts.jpg)

(In eckigen Klammern wird die Koroutinen-ID angezeigt.)


## A4.2: Warteschlange
Der Scheduler benötigt eine Warteschlange (engl. queue) bei der immer am Anfang einer einfach verketteten Liste ein Element entfernt wird (Thread der als nächstes die CPU erhält) und immer Ende eingefügt wird (zum Beispiel ein neuer Thread oder ein Thread der die CPU abgibt).

Implementieren Sie eine Warteschlange die diese Funktionalität realisiert im Verzeichnis `lib`. Testen Sie diese Klasse bevor Sie die nächste Aufgabe beginnen mit einem eigenen Testprogramm, außerhalb des Betriebssystems.

In folgender Datei muss Code implementiert werden: `lib/Queue.cc`.


## A4.3: Umbau der Koroutinen auf Threads
Kopieren Sie das Unterverzeichnis `kernel/corouts` um nach `kernel/threads` und benennen Sie danach die Dateien im Verzeichnis `kernel/threads` wie folgt um. Passen Sie dann die Namen der Klassen, Konstruktoren, Methoden und Funktionen in den obigen Dateien entsprechend an und ersetzen den Namen *Coroutine* durch den Namen *Thread*.

Umzukopieren sind folgende Dateien:
- Coroutine.asm -> Thread.asm 
- Coroutine.cc  -> Thread.cc

Modifizieren Sie nun `Thread.cc` passend zur der Datei `Thread.h` in der Vorgabe. 
- Die Methode `setNext` soll gelöscht werden.
- Die Methode `switchToNext` soll durch `switchTo` (siehe Vorgabe `Thread.h`) ersetzt werden. Die Semantik bleibt gleich, aber die Threads sind nicht fest miteinander verkettet, wie die Koroutinen. Daher wird der nächste Thread in `next` als Parameter übergeben.
- Im Konstruktor soll eine eindeutige Thread-ID vergeben werden, dies kann mithilfe einer globalen Variablen in `Thread.cc` realisiert werden. Zudem soll im Konstruktor der Stack (4 KB sind ausreichend) mithilfe ihrer Speicherverwaltung dynamisch angelegt werden.

*Hinweis: Diese Aufgabe kann nicht separat getestet werden.*


## A4.4 Scheduler
Nun soll ein einfacher Scheduler implementiert werden. Alle Threads werden in einer Bereit-Wartschlange (siehe A4.2) verwaltet und bekommen reihum die CPU (nach freiwilliger Abgabe mittels `yield`). Es gibt keine Prioritäten und es ist sinnvoll, dass der aktuell laufende Thread nicht in der Warteschlange gespeichert wird. In der Vorgabe ist die Implementierung für den Idle-Thread gegeben, welcher läuft, falls kein Anwendungsthread in der Bereit-Warteschlange ist. Der Idle-Thread soll im Rahmen der Initialisierung des Schedulers erzeugt und registriert werden, siehe A4.5. 

Testen Sie den Scheduler zunächst nur mit dem Idle-Thread. Bauen Sie hierzu eine Textausgabe in den Idle-Thread ein.

Gegeben ist in der Vorgabe die Klasse `Dispatcher`. Hier wird der aktuell laufende Thread verwaltet, sowie das Umschalten auf einen anderen Thread mithilfe der `switchTo`-Methode der Klasse `Thread` realisiert. In der Klasse `Dispatcher` ist auch eine Methode `start` zum Anstoßen des ersten Threads (mithilfe ihrer Assembler-Funktion).

In der gegebenen Datei `Scheduler.cc` sind die gekennzeichneten Methoden zu implementieren. Bei
einem Thread-Wechsel soll der Thread am Kopf der `readyQueue` entfernt und durch `life` (in
`Disptacher.cc`) referenziert. Gibt ein Thread die CPU freiwillig durch Aufruf von `yield` ab, soll dieser Thread wird wieder am Ende der `readyQueue` eingefügt werden.


## A4.5 Eine multi-threaded Testanwendung
Die Vorgabe beinhaltet einen HelloWorld-Thread, um einen ersten Test durchzuführen. Der Thread gibt einen Spruch aus und terminiert sich dann. Anschließend soll nur noch der Idle-Thread ausgeführt werden. Um dies zu testen soll der Idle-Thread und der HelloWorld-Thread in `main` angelegt und im Scheduler registriert werden. Anschließend soll der Scheduler mit `scheduler.schedule()` gestartet werden.

Als zweiter eigener Test soll nun eine multi-threaded Testanwendung bestehend aus vier Threads geschrieben werden. Hierzu soll das Anwendungsbeispiel mit den drei Zählern vom letzten Übungsblatt von Koroutinen auf Threads umgebaut werden. Ein Haupt-Thread der Anwendung `CoopThreadDemo` erzeugt drei Zähler-Threads, Instanzen von der Klasse `LoopThread`. Der Haupt-Thread der Anwendung soll eine gewisse Zeit laufen und sich dann selbst mit exit terminieren, nachdem er beispielsweise 1000 Mal die CPU bekommen hat. Bevor sich der Haupt-Thread der Anwendung terminiert soll er noch einen `LoopThread` mit `kill` terminieren. Somit sollten zunächst drei Zähler auf dem Bildschirm ausgegeben werden und dann einer bei 1000 stoppen, siehe Abbildung unten.

**Beispielausgaben der Threads**

![THR](https://github.com/mschoett/hhuTOSc/blob/aufgabe-4/img/threads.jpg)
