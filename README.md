# Aufgabe 6: Semaphore

## Lernziele
1. Verstehen wie Semaphoren implementiert werden und damit Thread-Synchronisierung funktioniert
2. Erweitern des Schedulers sowie der Thread-Zustände, um Blockieren und Deblockieren zu realisieren


## A6.1: Synchronisierung mit Interrupt-Sperre
Erweitern Sie das Testprogramm aus A5.4, indem Sie zwei oder drei Threads starten, welche jeweils einen Zähler an einer festen Position auf dem Bildschirm ausgeben. Zusätzlich soll ein weiterer Thread eine Melodie über den Lautsprecher abspielen.

Sie sollten dann beobachten können, dass die Ausgabe der Zähler nicht wie geplant funktioniert. Überlegen Sie warum dies so ist und synchronisieren Sie die Text-Ausgaben in den Threads durch einen kritischen Abschnitt, den Sie mithilfe von Interrupt-Sperren realisieren.
 
*Achtung: Das Sperren von Interrupts zu Synchronisierungszwecken funktioniert nur auf einem Einkern-BS!*


## A6.2: Semaphore
Unser Betriebssystem soll nun um Semaphore erweitert werden, mit denen Threads sich gegenseitig synchronisieren können, ohne Interrupts zu sperrern. Hierfür muss die Klasse `lib/Semaphore` implementiert werden. Jedes Semaphore-Objekt hat eine Warteschlange, in der Threads verwaltet werden, die blockiert sind, weil sie auf einen `v`-Aufruf für diese Semaphore warten. Die Methoden `p`und `v`müssen atomar ausgeführt werden, was mithilfe der Klasse  `lib/Spinlock` realisiert werden soll. Die Klasse `lib/Spinlock` ist in der Vorgabe fertig implementiert und bietet objekt-orientierte Sperren an, basierend auf der atomaren Maschineninstruktion `cmpxchg`. 

Zusätzlich muss der bestehende Scheduler um die Methoden `block` und `deblock` erweitert werden. In der Methode `block`soll auf den nächsten Thread umgeschaltet werden. Der aktuelle Thread soll nicht mehr in die `readyQueue` des Schedulers eingefügt werden, sondern wird in der Warteschlange der Semaphore verwaltet. In der Methode `deblock` soll der Thread der deblockiert werden soll, wieder in die `readyQueue` des Schedulers eingefügt werden. Wichtig ist, dass 
`block` und `deblock`, wie die anderen Methoden des Schedulers, gegenüber den Interrupts synchronisiert werden, da hier die `readyQueue` verändert wird.

Synchronisieren Sie die Zähler-Threads im Testprogramm aus A6.1 nun mithilfe eines Semaphor-Objektes. 

In folgenden Dateien muss Code implementiert werden: `lib/Semaphore.cc`, `kernel/Scheduler.cc`, `user/aufgabe6/SemaphoeDemo`und `user/aufgabe6/SemaLoopThread.cc`. 
