
/*****************************************************************************
 * Funktion:        force_unlock                                             *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Wird vom PIT gerufen, wenn ein erzwungener Thread-Wechsel*
 *                  durchgefuehrt wird. In diesem Fall wird aus der ISR      * 
 *                  heraus umgeschaltet, weswegen dann die Sperre auf        *
 *                  INT_VECTORS nicht freigegben wird. Dann wird die ISR     *
 *                  spaeter nicht mehr aufgerufen.                           *              
 *****************************************************************************/
pub fn force_unlock() {
   unsafe {
      INT_VECTORS.force_unlock();
   }
}

