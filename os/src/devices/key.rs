/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: key                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Defines a key consisting of ascii- and scan-code as well as     ║
   ║         modifiers. In addition helper functions for the modifiers are   ║
   ║         implemented.                                                    ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 6.2.2024                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

// Modifier bits
const KMOD_SHIFT: u8       = 1;
const KMOD_ALT_LEFT: u8    = 2;
const KMOD_ALT_RIGHT: u8   = 4;
const KMOD_CTRL_LEFT: u8   = 8;
const KMOD_CTRL_RIGHT: u8  = 16;
const KMOD_CAPS_LOCK: u8   = 32;
const KMOD_NUM_LOCK: u8    = 64;
const KMOD_SCROLL_LOCK: u8 = 128;

pub const SCAN_F1: u8 = 0x3b;
pub const SCAN_DEL: u8 = 0x53;
pub const SCAN_UP: u8 = 72;
pub const SCAN_DOWN: u8 = 80;
pub const SCAN_LEFT: u8 = 75;
pub const SCAN_RIGHT: u8 = 77;
pub const SCAN_DIV: u8 = 8;

/// Struct representing a key.
#[derive(Copy, Clone, Default)]
pub struct Key {
    asc: u8,  // ASCII code
    scan: u8, // scan code
    modi: u8, // modifier
}

impl Key {
    /// Create a new key with the given ASCII code, scancode and modifier.
    pub const fn new(asc: u8, scan: u8, modi: u8) -> Key {
        Key { asc, scan, modi }
    }

    /// Invalid keys are represented by a scancode of 0.
    pub fn valid(&mut self) -> bool {
        self.scan != 0
    }

    /// Make the key invalid by setting the scancode to 0.
    pub fn set_invalidate(&mut self) {
        self.scan = 0;
    }

    // Functions for manipulating ASCII and scancode of the key
    pub fn set_ascii(&mut self, a: u8) { self.asc = a; }
    pub fn set_scancode(&mut self, s: u8) { self.scan = s; }
    pub fn get_ascii(&mut self) -> u8 { self.asc }
    pub fn get_scancode(&mut self) -> u8 { self.scan }

    // Functions for manipulating the modifier bits
    pub fn set_shift(&mut self, pressed: bool) {
        if pressed == true { self.modi = self.modi | KMOD_SHIFT; }
        else               { self.modi = self.modi & !KMOD_SHIFT; }
    }

    pub fn set_alt_left(&mut self, pressed: bool) {
        if pressed == true { self.modi = self.modi | KMOD_ALT_LEFT; }
        else               { self.modi = self.modi & !KMOD_ALT_LEFT; }
    }

    pub fn set_alt_right(&mut self, pressed: bool) {
        if pressed == true { self.modi = self.modi | KMOD_ALT_RIGHT; }
        else               { self.modi = self.modi & !KMOD_ALT_RIGHT; }
    }

    pub fn set_ctrl_left(&mut self, pressed: bool) {
        if pressed == true { self.modi = self.modi | KMOD_CTRL_LEFT; }
        else               { self.modi = self.modi & !KMOD_CTRL_LEFT; }
    }

    pub fn set_ctrl_right(&mut self, pressed: bool) {
        if pressed == true { self.modi = self.modi | KMOD_CTRL_RIGHT; }
        else               { self.modi = self.modi & !KMOD_CTRL_RIGHT; }
    }

    pub fn set_caps_lock(&mut self, pressed: bool) {
        if pressed == true { self.modi = self.modi | KMOD_CAPS_LOCK; }
        else               { self.modi = self.modi & !KMOD_CAPS_LOCK; }
    }

    pub fn set_num_lock(&mut self, pressed: bool) {
        if pressed == true { self.modi = self.modi | KMOD_NUM_LOCK; }
        else               { self.modi = self.modi & !KMOD_NUM_LOCK; }
    }

    pub fn set_scroll_lock(&mut self, pressed: bool) {
        if pressed == true { self.modi = self.modi | KMOD_SCROLL_LOCK; }
        else               { self.modi = self.modi & !KMOD_SCROLL_LOCK; }
    }

    // Functions for reading the modifier bits
    pub fn get_shift(&self) -> bool       { (self.modi & KMOD_SHIFT) != 0          }
    pub fn get_alt_left(&self) -> bool    { (self.modi & KMOD_ALT_LEFT) != 0       }
    pub fn get_alt_right(&self) -> bool   { (self.modi & KMOD_ALT_RIGHT) != 0      }
    pub fn get_ctrl_left(&self) -> bool   { (self.modi & KMOD_CTRL_LEFT) != 0      }
    pub fn get_ctrl_right(&self) -> bool  { (self.modi & KMOD_CTRL_RIGHT) != 0     }
    pub fn get_caps_lock(&self) -> bool   { (self.modi & KMOD_CAPS_LOCK) != 0      }
    pub fn get_num_lock(&self) -> bool    { (self.modi & KMOD_NUM_LOCK) != 0       }
    pub fn get_scroll_lock(&self) -> bool { (self.modi & KMOD_SCROLL_LOCK) != 0    }
    pub fn get_alt(&self) -> bool { self.get_alt_left() || self.get_alt_right()    }
    pub fn get_ctrl(&self) -> bool { self.get_ctrl_left() || self.get_ctrl_right() }
}

