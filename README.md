# Aufgabe 13: Verwaltung virtueller Adressräume (Isolation & Schutz - Aufgabe 6)

## Lernziele
1. Virtuelle Adressräume sollen abstrack durch Virtual Memory Areas (VMAs) verwaltet werden.
2. User Mode Anwendungen bekommen einen eigenen Heap.
3. User Stacks werden nicht direkt vollständig alloziert, sondern wachsen bei Bedarf.

## A13.1: Virtual Memory Areas (VMAs)
n dieser Aufgabe soll der virtuelle Adressraum eines Prozesses abstrakt mithilfe von Virtual Memory Areas (VMAs), ähnlich wie unter Linux, verwaltet werden. Eine VMA ist eine Region im virtuellen Adressraum, hat also eine Anfangs- und Endadresse (beide seitenaligniert) sowie einen Typ (Code, Stack, Heap) und eine VMA gehört immer zu genau einem Prozess.

Das Paging wird wie bisher verwendet, um separate Adressräume zu realisieren sowie den Speicherschutz des Kernel-Adressbereichs sicherzustellen.

Die VMAs eines Prozesses soll als `Vec` im `Process`-Struct gespeichert werden. Implementieren Sie zusätzlich die Funktion `process::add_vma(process_id: usize, vma: VMA)` um dem Prozess eine neue VMA hinzuzufügen. Diese Funktion soll prüfen, ob die neue VMA mit einer existierenden VMA überlappt und falls ja, einen Fehler zurückgeben und abbrechen. Falls keine Überlappung vorliegt, soll die neue VMA in der VMA-Liste des Prozesses gespeichert werden.

Für jeden Prozess soll initial eine VMA für den Code-Bereich (vom Start des User Mode Bereichs bis zum Ende des Codes der Anwendung) und eine VMA für den Stack (letzte Seite des User Mode Bereichs) angelegt werden.

Damit die VMAs eines Prozesses ausgegeben werden können soll ein neuer Systemaufruf `usr_dump_vmas()` implementiert und in einer Test-Anwendung ausprobiert werden. Dieser Aufruf soll mit `kprintln!()` alle VMAs des aufrufenden Prozesses ausgeben.

Weitere Informationen zu VMAs in Linux sind hier gut erläutert: https://manybutfinite.com/post/how-the-kernel-manages-your-memory/ 

## A13.2: Ein Heap für Prozesse
In dieser Aufgabe soll jeder Prozess einen eigenen User Mode Heap bekommen. Der Heap soll als VMA im virtuellen Adressraum des Prozesses verwaltet werden. Wir verwenden hierzu unseren existierenden Allokator und nutzen diesen nun für den Kernel und die User Anwendungen.

Verschieben Sie hierfür zunächst den Ordner `kernel/allocator/` und die Datei `kernel/allocator.rs` nach `usrlib`. Nemen Sie anschließend alle nötigen Änderungen am Code vor, damit das Betriebssystem weiterhin kompiliert und läuft (z.B. Anpassung von includes).

Der Speicherbereich, den der Allokator verwaltet soll über einen neuen Systemaufruf `usr_map_heap(user_heap_start: u64, user_heap_size: usize)` angefordert werden. Hierzu wird eine weitere Funktion `map_user_heap(pml4_table: &mut PageTable, user_heap_start: u64, user_heap_size: usize)` in `pages.rs` benötigt. Der allozierte Adressbereich soll im User Mode Adressbereich liegen und für die benötigten Page Frames des Heaps sollen vom dem Page Frame Allokator angefordert werden.

Test Sie den neuen Heap in einer User Mode Anwendung, indem Sie den Systemaufruf `usr_map_heap()` aufrufen und anschließend dynamisch Speicher allozieren (z.B. mit `Box::new()` oder `Vec::with_capacity()`).

## A13.3: Dynamische Vergrößerung des User Stacks
Bisher wurde der Stack für jeden User-Thread mit einer festen Größe alloziert, dies soll nun angepasst werden, sodass der Stack bei Bedarf dynamisch wächst. Hierzu sind keine Änderungen im User-Mode notwendig, jedoch im Kernel.

Die Funktion `pages::map_user_stack()` soll so angepasst werden, dass nur die oberste Seite des Stacks alloziert und eingeblendet wird (die VMA soll aber weiterhin die gesamte Stack-Größe abdecken).
Wächst der Stack nun über eine Größe von 4 KiB hinaus, sollte eine Page Fault ausgelöst werden. Testen Sie dies, indem Sie in einer User Mode Anwendung eine große Menge an Stack-Speicher allozieren. Hierfür bietet sich eine rekursive Funktion an, die am besten viele Parameter hat (oder ein großes Struct als Parameter übergeben bekommt). Zum Beispiel können Sie die Fibonacci-Funktion mit zusätzlichen ungenutzten Parametern implementieren.

Erweitern Sie anschließend den Page Fault Handler in `interrupts.rs` so, dass geprüft wird, ob die Page Fault Adresse innerhalb des virtuellen Speicherbereichs für den User Stack liegt. Falls ja, soll für die betroffene Page ein Page Frame alloziert und eingeblendet werden, sodass der Stack wächst. Hierzu bietet sich die Implementierung einer Hilfsfunktion `check_and_grow_user_stack()` in `pages.rs` an, die im Page Fault Handler aufgerufen wird.
