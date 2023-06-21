# Aufgabe 7: Eine eigene BS-Erweiterung / Anwendung

## Lernziele
1. Eine Anwendung schreiben
2. Alternativ eine Betriebssystem-Komponente entwickeln

## Mögliche Themenrichtunge
- Grafikdemo (multithreaded)
- Retro-Spiel (z.B. Snake, Pacman, ...)
- einfache Shell (Beispiele für Befehle: clear, time, meminfo, ...) 
- Scheduler mit Prioriäten (mit einer Demo)


## Vorgabe
Die Dateien in der Vorgabe umfassen einige Dateien, um einen Grafikmodus nutzen zu können. Die Vorgabe ist nur notwendig, sofern Sie im Grafikmodus arbeiten möchten.

### Grafikfunktionen 
Vorhanden sind nur sehr grundlegende Grafik-Funktionen, inkl. einer Text-Ausgabe mit einer Schriftart. Weitere Funktionen sollen je nach Anwendung ergänzt werden. 

Ob das System im Grafik- oder Textmodus startet wird in `boot/boot.asm`durch die die Konstante `TEXT_MODE` festgelegt. Wenn diese Konstante aukommentiert wird, so schaltet `grub` direkt in den Grafikmodus (800x600 mit 32 Bit pro Pixel). Eine alternative Grafikauflösung kann durch die Konstanten `MULTIBOOT_GRAPHICS_*` in  `boot/boot.asm` eingestellt werden. Mögliche Auflösungen sollten sich an dem VESA-Standard orientieren, siehe hier: https://en.wikipedia.org/wiki/VESA_BIOS_Extensions

Die Textausgabe über CGA funktioniert nicht im Grafikmodus! Damit die Textausgaben wieder erscheinen muss in `Globals` die Variable `kout`auf `VGA_Stream`umgestellt werden, siehe Vorgabe.

Zum Testen ist eine kleine Demo in `user/aufgabe7/GraphicDemo.cc`, siehe auch nachstehendes Bild. 

Folgende Dateien sind für die Grafik-Unterstützung in der Vorgabe:
- `VGA`: Zeichenfunktionen
- `VGA_Stream`: Textausgabe über den Stream-Operator `<<` im Grafikmodus 
- `Globals.cc`: `kout` für  `VGA_Stream`
- `fonts/*`: Bitmap-Fonts für die Textausgabe im Grafikmodus
- `user/aufgabe7/GraphicDemo`: Zeigt Ausgaben im Grafikmodus

**Beispielausgabe der Grafikdemo**

![GD](https://github.com/hhu-bsinfo/hhuTOSc/blob/aufgabe-7/img/graphic.jpg)
