# Aufgabe 10: Speicherverwaltung für physikalischen Speicher (Isolation und Schutz - Aufgabe 3)

## Lernziele
1. Eine Speicherverwaltung für physikalische Kacheln (Page Frames) implementieren.

## A10.1: Verfügbaren physikalischen Speicher ermitteln
Bevor das Paging implementiert wird, muss zunächst ermittelt werden, wie viel und wo nutzbarer physikalischer Speicher vorhanden ist. Die Lösung ist in `multiboot.rs` in der Funktion `init_phys_memory_allocator()` bereits vorhanden, soll aber gelesen und verstanden werden.

Die notwendigen Informationen bekommen wir von Multiboot in Form von `mmap`-Einträgen. Diese Einträge beschreiben jeweils einen zusammenhängenden Block physikalischen Speichers, der entweder reserviert oder verfügbar sein kann. Die als verfügbar markierten Speicherbereiche werden in unsere Speicherverwaltung für physikalischen Speicher via `PfListAllocator::free_block()` eingefügt. Wir müssen jedoch beachten, dass der Kernel-Code vom Bootloader nicht als reserviert markiert wird. Unser Kernel-Image wird an die Adresse 1 MiB geladen. Der Linker erzeugt die Symbole `___KERNEL_DATA_START__` und `___KERNEL_DATA_END__` um diesen Speicherbereich zu markieren. Außerdem wird der Bereich von 0 bis 1 MiB teilweise vom BIOS verwendet (Der Grafikspeicher liegt auch hier), weshalb wir ihn als nicht nutzbar betrachten. Der für uns nutzbare physikalische Speicher beginnt also direkt hinter dem Kernel Image.

Fügen Sie den folgenden Code zu Beginn ihrer `startup()`-Methode in `boot.rs` ein, um die physikalische Speicherverwaltung mit den freien Speicherblöcken aus der Multiboot Memory Map zu initialisieren:
```
// Copy multiboot into on stack, because it lies in physical memory that might get reused after initializing the physical memory allocator
let multiboot_info = *multiboot_info;

kprintln!("Initializing physical memory allocator");
multiboot_info.init_phys_memory_allocator();
```

*Information zu Multiboot, insbesondere den mmap-Einträgen finden Sie hier:* https://www.gnu.org/software/grub/manual/multiboot/multiboot.html

## A10.2: Ein Allokator für Page Frames
Der verfügbare physikalische Speicher muss in 4 KiB Kacheln (Page Frames) verwaltet werden. Hierfür wird nun ein Page-Frame-Allokator benötigt. Als Basis empfiehlt sich der vorhandene Heap-Allokator (in list.rs), der Code muss natürlich angepasst werden. Wir verketten also die freien Page-Frames, wobei ein Block aus mehreren aufeinanderfolgenden Page-Frames bestehen kann. Die Metadaten für freie Blöcke schreiben wir direkt in die freien Page-Frames. Für die belegten Page-Frames be­nötigen wir keine Metadaten, da es nur 4 KiB Page-Frames gibt und jeder Page-Frame von einem Seiten­tabellen-Eintrag referenziert wird, sobald wir Paging implementiert haben.

Beim Freigeben eines Speicherblocks soll dieser **sortiert** in die Liste eingefügt werden. Falls ein freigegebener Block an seinen Vorgänger oder Nachfolger angrenzt, soll er mit diesem (oder falls möglich beiden) verschmolzen werden. So beugen wir einer Fragmentierung des Speichers in viele kleine Blöcke vor.

Schreiben sie eine Testfunktion, die die Korrektheit ihres Allokators überprüft. Hierfür ist es nützlich eine `dump()` Funktion zu implementieren, welche die gesamte Freispeicherliste auf dem Bildschirm oder seriellen Port ausgibt.

*Wichtige Hinweise:*
 - *Der Allokator darf keinesfalls einen reservierten Page-Frame erneut vergeben! Bevor Sie fortfahren, sollten Sie die Korrektheit Ihres Page-Frame-Allokators testen! `assert!()` ist hier sehr nützlich.*
 - *Ferner empfiehlt es sich Page-Frames beim Allozieren mit 0 zu initialisieren. So können später Pointer-Fehler einfacher erkannt werden, da dann ein Null-Pointer-Zugriff mithilfe des Pagings erkannt werden kann.*

## A10.3: Kernel Heap
Nun soll noch die Initialisierung des bestehenden Allokators für den Heap in `startup.rs` angepasst werden. Statt an einer festen Adresse einfach freien Speicher zu vermuten, soll nun freier Speicher vom Page-Frame-Allokator für den Heap angefordert werden. Nun sollte alles weiterhin funktionieren. Die Stacks werden vorerst noch im Kernel-Heap alloziert. Dies ändert sich mit dem nächsten Aufgabenblatt, wenn wir Paging einführen.
