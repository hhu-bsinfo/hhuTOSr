# Aufgabe 12: Separat kompilierte Anwendungen & Prozesse

## Lernziele
1. Kernel- und User-Code separat voneinander kompilieren.
2. Den Kernel vor Zugriff aus dem User Space schützen.
3. Eine einfach Struktur zur Verwaltung von Prozessen einführen.

## A12.1: Getrennte Übersetzung von Anwendungen
In der Vorgabe finden Sie die neue Ordnerstruktur für Ihr Betriebssystem. Diese ist auf der obersten Ebene in drei Ordner aufgeteilt:
 - `os`: Enthält den gesamten Kernel-Quellcode
 - `apps`: Enthält den Quellcode für alle Anwendungen. Jede Anwendung ist dabei ein separates Rust-Projekt in einem eigenen Unterordner. Vorgegeben ist nur die Anwendung `hello`.
 - `usrlib`: Enthält Bibliotheksfunktionen, die von den Anwendung verwendet werden können.
 
 Machen Sie sich zunächst mit der Vorgabe vertraut. Unter `apps/hello` finden Sie eine Beispiel-Anwendung, die als eigenes Rust-Projekt separat vom Kernel kompiliert wird. Diese soll von unserem Betriebssystem zur Laufzeit in ihren eigenen Adressraum geladen und ausgeführt werden. Hierbei stellen sich zunächst einige Fragen:
  1. *Wie kommt unser Betriebssystem an die Anwendung?* Der Bootloader GRUB kann zusätzlich zum Kernel auch noch sogenannte Module für uns in den Arbeitsspeicher laden. Ein Modul ist dabei einfach eine beliebige Datei, die im Boot-Image hinterlegt ist. Wir packen alle unsere Anwendung zusammen in ein (unkomprimiertes) TAR-Archiv und lassen dieses vom Bootloader in den Arbetsspeicher laden. Unser Betriebssystem kann dann jederzeit auf Anwendungen in dem Archiv zugreigen und diese ausführen. Um das TAR-Archiv zu parsen nutzen wir die Crate `tar-no-std`.
  2. *Welches Dateiformat haben unsere Anwendungen und wie finden wir darin den Code?* Wir linken die Anwendungen zunächst im ELF-Format. Dieses ist jedoch recht komplex und unser Betriebssystem bräuchte einen eigenen Parser dafür. Wir nutzen daher das Programm `objcopy` um den Code aus der ELF-Teil zu extrahieren und in einer sogenannten *Flat Binary* zu speichern. Diese enthält dann ausschließlich den Code und erfordert kein spezielles Parsing. Wir müssen bei diesem Ansatz nur dafür sorgen, dass die `main()`-Funktion einer Anwendung immer direkt am Anfang der Datei steht. Dafür versehen wir die `main()`-Funktion mit `#[unsafe(link_section = ".main")]` und erzeugen so eine eigene Sektion für diese. Im Linker-Skript sorgen wir dann dafür, dass die `main`-Sektion immer an den Anfang des Codes gelinkt wird.
  3. *Wie können Anwendungen auf gemeinsame Bibliotheksfunktionen zugreifen, ohne dass wir doppelten Code erzeugen?* Dazu werden alle Bibliotheksfunktionen im Ordner `usrlib` implementiert. Jede Anwendung hat in ihrer `Makefile.toml` eine Abhängigkeit zu diesem Ordner. Das sorgt dafür, dass der Code aus `usrlib` mit jeder Anwendung kompiliert und statisch gelinkt wird.

Um mit der neuen Ordnerstruktur zu starten, müssen Sie Ihren bisherigen Quellcode nach `os/src` kopieren. Anschließen müssen die Dateien `user_api.rs` und `spinlock.rs` nach `usrlib/src` kopiert werden. Dies erfordert leiche Anpassungen an Ihrem System, überall dort wo `Spinlock` importiert wird. Die Importe müssen so angepasst werden, dass sie nun das Spinlock aus `usrlib` importieren. Das Betriebssystem hat in seiner `Makefile.toml` bereits eine Abhängigkeit zur `usrlib`, so dass es dabei keine größeren Probleme geben sollte.

In `usrlib/print.rs` sind bereits die Makros `pint!()` und `println!()` für den User-Space implementiert. Diese erwarten, dass der System Call `usr_print(msg: &str)` in `user_api.rs` implementiert ist. Dieser soll einen String an der aktuellen Cursor-Position ausgeben. Sie können die Vorgabe natürlich auch anpassen, falls Ihr System Call zur Textausgabe anders aussieht.

In `os/src/boot/grub.cfg` muss außerdem noch die Zeile `module /boot/initrd.tar` unter `multiboot /boot/kernel.bin` ergänzt werden, damit der Bootloader das TAR-Archiv lädt.

Beim Kompielieren ist nun der zusätzliche Parameter `--no-workspace` notwendig. Dieser sorgt dafür, dass das Build-System nicht die `Makefile.toml` jedes Unterprojekts einzeln ausführt, sondern nur die `Makefile.toml` im Wurzelverzeichnis unserer Projektstruktur beachtet. Diese wiederum hat Abhängigkeiten zu den `link`-Tasks der einzelnen Anwendungen und stellt so sicher, dass das Boot-Image erst gebaut wird, wenn alle Anwendungen kompiliert und gelinkt wurden. Der vollständige Befehl zum Starten des Systems lauten nun:

```
cargo make --no-workspace qemu
```

## A12.2: Ein Mapping für das Anwendungsimage
In der vorgegeben `multiboot.rs` finden Sie zwei Funktionen, die Sie in Ihre `multiboot.rs` kopieren sollen. Diese sind dafür zuständing, das TAR-Archiv mit unseren Anwendungen zu finden. Mit `get_initrd_archive()` erhalten Sie sich eine Referenz auf das Archiv. Das dazugehörige Struct und dessen Implementierung kommen aus der Crate `tar-no-std`. Mit der `entries()`-Methode des `TarArchiveRef`-Structs erhält man einen Iterator über alle Dateien des Archivs und kann diese so z.B. mit einer for-each-Schleife durchsuchen.

Neue User-Threads sollen nun immer eine Anwendung aus dem TAR-Archiv ausführen. Dazu muss physikalischer Speicher in der passenden Größe alloziert und die Anwenndung dorthin kopiert werden. Anschließend muss Sie dieser physikalische Speicher in den Adressraum der Anwendung eingeblendet werden. Wir verwenden hierzu die Adresse `0x100_0000_0000`, was 1 TiB entspricht (siehe `consts.rs`). Hierfür bietet es sich an, eine weitere Funktion `map_user_app()` in `pages.rs` zu implementieren. Die `entry`-Funktion eines User-Threads soll nun einfach auf diese feste virtuelle Adresse verweisen. Mit `core::mem::transmute()` lässt sich die Konstante in den Typ `fn()` umwandeln (*ACHTUNG: Das ist unsafe und sollte nur in Ausnahmefällen genutzt werden!*).

Da die Anwendungen nun nicht mehr Teil des Kernel-Images sind, lassen sie sich nicht direkt debuggen, da der Debugger die Symbole der Anwendnungen nicht kennt. Um das zu ändern, können Sie nach dem Starten des Debuggers das System anhalten und in der GDB-Konsole mit dem Befehl `add-symbol-file target/hhu_tosr_app/debug/hello.elf` die Anwendungssymbole nachladen. *ACHTUNG: Es sollte immer nur eine Anwendung auf einmal geladen werden, da sie sich die virtuellen Adressen der Anwendungen überlappen*.

*Hinweis: Aus der main()-Funktion einer Anwendung kann man nicht mehr zurückkehren. Anwendungen müssen per System Call (`usr_thread_exit()`) beendet werden.

## A12.3: Kernel-Space schützen
Bisher kann jede Anwendung auf den Kernel-Speicherbereich (Adressen kleiner als 1 TiB) zugreifen (lesend und schreibend). Nun soll der Kernel über das Paging geschützt werden, in dem bei den entsprechenden Seitentabellen-Einträgen das User-Bit (U/S) gelöscht wird.

Anschließend muss noch der Startvorgang eines User-Threads angepasst werden. Wir können nun von `thrad_user_start()` nicht mehr nach `kickoff_user_thread()` springen, da diese Funktion Teil des Kernels ist und somit im User-Mode nicht mehr ausgeführt werden kann. Wir müssen daher in `switch_to_usermode()` nun direkt die `entry` Adresse eines User-Threads auf den vorbereiteten Stack legen, statt der Adresse von `kickoff_user_thread()`.

## A12.4: Prozesse
In dieser Aufgabe wird eine Verwaltungsstruktur für Prozesse implementiert sowie der Start von Threads umgebaut, sodass nun Prozesse mit jeweils einem Thread gestartet werden. 

Alle laufenden Prozesse sollen in `processes/process.rs` in einem Key-Value-Baum (`BTreeMap`) verwaltet werden. Als Key dient die Prozess-ID und als Value wird die Prozess-Struktur `Process` gespeichert. Hierdurch können Prozessinformationen später schnell über die pid abgerufen werden. Die Baumstruktor gibt es fertig in der Crate `alloc` (siehe Vorgabe).

In `thread.rs` sind kleinere Anpassungen notwendig. Im `Thread`-Struct wird nun auch die Prozess-ID gespeichert, damit jeder Thread die Zuordnung zu seinem Prozess kennt.

In `scheduler.rs` soll eine Funktion `spawn_process()` implementiert werden, welche den Namen einer Anwendung entgegennimmt, einen Prozess dafür anlegt, und sie in einem User-Thread startet. Wenn eine Anwendung beendet wird (`Scheduler::exit()`), soll außerdem auch der entsprechende Prozess aus der `BTreeMap` gelöscht werden.

Zuletzt soll ein System Call implementiert werden, der die Prozess-ID des aktuell laufenden Threads zurückgibt. Testen Sie Ihr System außerdem, in dem Sie verschiedene System Calls von einer Anwendung aus aufrufen.
