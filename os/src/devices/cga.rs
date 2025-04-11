/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: cga                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: This module provides functions for doing output on the CGA text ║
   ║         screen. It also supports a text cursor position stored in the   ║
   ║         hardware using ports.                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 6.2.2024                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use spin::Mutex;
use crate::kernel::cpu as cpu;

/// Global CGA instance, used for screen output in the whole kernel.
/// Usage: let mut cga = cga::CGA.lock();
///        cga.print_byte(b'X');
pub static CGA: Mutex<CGA> = Mutex::new(CGA::new());

/// All 16 CGA colors.
#[repr(u8)] // store each enum variant as an u8
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

pub const CGA_STD_ATTR: u8 = (Color::Black as u8) << 4 | (Color::Green as u8);

const CGA_BASE_ADDR: *mut u8 = 0xb8000 as *mut u8;
const CGA_ROWS: usize = 25;
const CGA_COLUMNS: usize = 80;

const CGA_INDEX_PORT: u16 = 0x3d4; // select register
const CGA_DATA_PORT: u16 = 0x3d5;  // read/write register
const CGA_HIGH_BYTE_CMD: u8 = 14;  // cursor high byte
const CGA_LOW_BYTE_CMD: u8 = 15;   // cursor high byte

pub struct CGA {
    index_port: cpu::IoPort,
    data_port: cpu::IoPort
}

impl CGA {
    /// Create a new CGA instance.
    const fn new() -> CGA {
        CGA {
            index_port: cpu::IoPort::new(CGA_INDEX_PORT),
            data_port: cpu::IoPort::new(CGA_DATA_PORT)
        }
    }

    /// Clear CGA screen and set cursor position to (0, 0).
    pub fn clear(&mut self) {
        /* Hier muss Code eingefuegt werden */
    }

    /// Display the `character` at the given position `x`,`y` with attribute `attrib`.
    pub fn show(&mut self, x: usize, y: usize, character: char, attrib: u8) {
        if x > CGA_COLUMNS || y > CGA_ROWS {
            return;
        }

        let pos = (y * CGA_COLUMNS + x) * 2;

        // Write character and attribute to the screen buffer.
        //
        // Unsafe because we are writing directly to memory using a pointer.
        // We ensure that the pointer is valid by using CGA_BASE_ADDR
        // and checking the bounds of x and y.
        unsafe {
            CGA_BASE_ADDR.offset(pos as isize).write(character as u8);
            CGA_BASE_ADDR.offset((pos + 1) as isize).write(attrib);
        }
    }

    /// Return cursor position `x`,`y`
    pub fn getpos(&mut self) -> (usize, usize) {
        /* Hier muss Code eingefuegt werden */

        (0, 0) // Platzhalter, entfernen und durch sinnvollen Rueckgabewert ersetzen 
    }

    /// Set cursor position `x`,`y` 
    pub fn setpos(&mut self, x: usize, y: usize) {
        /* Hier muss Code eingefuegt werden */
    }

    /// Print byte `b` at actual position cursor position `x`,`y`
    pub fn print_byte(&mut self, b: u8) {
        /* Hier muss Code eingefuegt werden */
    }

    /// Scroll text lines by one to the top.
    pub fn scrollup(&mut self) {
        /* Hier muss Code eingefuegt werden */
    }

    /// Helper function returning an attribute byte for the given parameters `bg`, `fg`, and `blink`
    /// Note: Blinking characters do not work in QEMU, but work on real hardware.
    ///       Support for blinking characters is optional and can be removed, if you want.
    pub fn attribute(&mut self, bg: Color, fg: Color, blink: bool) -> u8 {
        /* Hier muss Code eingefuegt werden */

        0 // Platzhalter, entfernen und durch sinnvollen Rueckgabewert ersetzen 
    }
}