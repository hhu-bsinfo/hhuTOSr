
use nolock::queues::mpmc;
use nolock::queues::mpmc::bounded::scq::{Receiver, Sender};

/// Global keyboard instance.
pub static KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard::new());

/// Global key buffer.
/// Each key is pushed to this queue by the interrupt handler
/// and can be retrieved at a later time by the user.
/// Wrapped inside a Once, because the Queue cannot be created inside a const function.
static KEYBOARD_BUFFER: Once<KeyQueue> = Once::new();

/// Global access to the key buffer.
/// Usage: let key_buffer = keyboard::get_key_buffer();
///        let key = key_buffer.get_last_key();
pub fn get_key_buffer() -> &'static KeyQueue {
    KEYBOARD_BUFFER.call_once(|| {
        KeyQueue::new()
    })
}

/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Interrupt service routine implementation.                               ║
   ╚═════════════════════════════════════════════════════════════════════════╝ */

/// Register the keyboard interrupt handler.
pub fn plugin() {

    /* Hier muss Code eingefuegt werden */

}

/// The keyboard interrupt service routine.
pub struct KeyboardISR {}

impl ISR for KeyboardISR {
    fn trigger(&self) {

        /* Hier muss Code eingefuegt werden */

    }
}

/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Key buffer implementation.                                              ║
   ╚═════════════════════════════════════════════════════════════════════════╝ */

/// Represents a first in first out queue for keyboard keys.
/// It uses a multi-producer multi-consumer queue from the nolock crate,
/// allowing thread safe access without needing a Mutex.
pub struct KeyQueue {
    /// Keys can be popped from the queue via the receiver.
    receiver: Receiver<Key>,
    /// Keys can be pushed to the queue via the sender.
    sender: Sender<Key>
}

impl KeyQueue {
    /// Create a new empty queue.
    /// Unfortunately, this cannot be done in a const function.
    fn new() -> KeyQueue {
        let (receiver, sender) = mpmc::bounded::scq::queue(128);
        KeyQueue { receiver, sender }
    }

    /// Push a key to the queue.
    /// If the queue is full, the key is silently discarded.
    pub fn push_key(&self, key: Key) {
        if self.receiver.is_closed() {
            // Should never haven
            panic!("KeyQueue is closed!");
        }

        // Enqueue the key into the queue.
        // If the queue is full, we ignore the key.
        self.sender.try_enqueue(key).ok();
    }

    /// Pop a key from the queue.
    /// If the queue is empty, None is returned.
    pub fn get_last_key(&self) -> Option<Key> {
        if self.receiver.is_closed() {
            // Should never haven
            panic!("KeyQueue is closed!");
        }

        match self.receiver.try_dequeue() {
            Ok(key) => Some(key),
            Err(_) => None
        }
    }

    /// Pop a key from the queue.
    /// If the queue is empty, the function blocks until a key is available.
    pub fn wait_for_key(&self) -> Key {
        if self.receiver.is_closed() {
            // Should never haven
            panic!("KeyQueue is closed!");
        }

        loop {
            match self.receiver.try_dequeue() {
                Ok(key) => return key,
                Err(_) => {}
            }
        }
    }
}

/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Implementation of the keyboard driver itself.                           ║
   ╚═════════════════════════════════════════════════════════════════════════╝ */

impl Keyboard {
    /// Poll a byte from the keyboard controller.
    /// Decode and return the key if it is complete.
    fn key_hit_irq(&mut self) -> Option<Key> {

        /* Hier muss Code eingefuegt werden */

        None
    }
}
