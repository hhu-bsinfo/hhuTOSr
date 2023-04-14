
pub fn get_initialized() -> bool {
	SCHEDULER.lock().initialized
}

pub fn set_initialized() {
    SCHEDULER.lock().initialized = true;
}



pub struct Scheduler {
   active: *mut thread::Thread,
   ready_queue: queue::Queue<Box<thread::Thread>>,   // auf die CPU wartende Threads
   next_thread_id: u64,
   initialized: bool,
}


impl Scheduler {
	
   // Scheduler mit Ready-Queue anlegen
   pub const fn new() -> Self {
	 Scheduler { active: ptr::null_mut(), 
	             next_thread_id:0, 
	             ready_queue: queue::Queue::new(),
	             initialized:false, 
	           }
   }

   /*****************************************************************************
    * Funktion:         preempt                                                 *
    *---------------------------------------------------------------------------*
    * Beschreibung:    CPU soll aktuellem Thread entzogen werden. Wird nur      *
    *                  aus dem Zeitgeber-Interrupt-Handler aufgerufen. Daher    *
    *                  muss nicht gegenueber Interrupts synchronisiert werden.  *
    *****************************************************************************/
   pub fn preempt (&mut self) {
  
      /* Hier muss Code eingefuegt werden */

   }


