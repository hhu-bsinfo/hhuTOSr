;*****************************************************************************
;*                                                                           *
;*                  C O R O U T I N E                                        *
;*                                                                           *
;*---------------------------------------------------------------------------*
;* Beschreibung:    Assemblerfunktionen zum Starten des ersten Koroutine und *
;*                  zum Umschalten zwischen Koroutinen.                      *
;*                                                                           *
;* Autor:           Michael, Schoettner, HHU, 14.03.2023                     *
;*****************************************************************************


; EXPORTIERTE FUNKTIONEN

[GLOBAL _coroutine_start]
[GLOBAL _coroutine_switch]

; IMPLEMENTIERUNG DER FUNKTIONEN

[SECTION .text]
[BITS 64]


; _coroutine_start: Startet die erste Koroutine
;
; C Prototyp:      void _coroutine_start(context: *mut c_void); 
_coroutine_start:

; * 
; * Hier muss Code eingefuegt werden
; * 

;        
; _coroutine_switch: Coroutinen-Umschaltung. Der aktuelle Registersatz wird
;                    auf dem Stack gesichert und der Registersatz der 
;                    neuen Coroutine wird in den Prozessor geladen.
;
; C Prototyp:       void _coroutine_switch (context_now: *mut c_void, context_then: *mut c_void);
_coroutine_switch:
; * 
; * Hier muss Code eingefuegt werden
; * 
