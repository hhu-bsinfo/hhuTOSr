;*****************************************************************************
;*                                                                           *
;*                  C O R O U T I N E                                        *
;*                                                                           *
;*---------------------------------------------------------------------------*
;* Beschreibung:    Assemblerfunktionen zum Starten der Koroutine und zum    *
;*                  Umschalten zwischen Koroutinen.                          *
;*                                                                           *
;* Autor:           Michael, Schoettner, HHU, 15.05.2023                     *
;*****************************************************************************


; EXPORTIERTE FUNKTIONEN

[GLOBAL _coroutine_start]
[GLOBAL _coroutine_switch]

; IMPLEMENTIERUNG DER FUNKTIONEN

[SECTION .text]
[BITS 64]

 
;
; fn _coroutine_start(now_coroutine_struct:  *mut c_void); 
;    Startet die erste Koroutine
;
;    'now_coroutine_struct' zeigt auf die Coroutine struct der zu 
;    startenden Coroutine. Um an 'context' innerhalb der struct zu 
;    gelangen muss auf die Adresse now_coroutine_struct +8 aufaddiert 
;    werden. Dann haben wir den zuletzt belegten Eintrag 
;    des praeparierten Stacks
;
_coroutine_start:

;
; Hier muss Code eingefuegt werden
;

;
; fn _coroutine_switch(now_coroutine_struct:  *mut c_void, 
;                      then_coroutine_struct: *mut c_void);
;    Umschalten zw. Coroutinen
;
;    Bzgl. der Parameter siehe Beschreibung von '_coroutine_start'
;
_coroutine_switch:

;
; Hier muss Code eingefuegt werden
;
