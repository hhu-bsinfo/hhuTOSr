/*
 * Module: cga
 *
 * Description: This module provides functions for doing output on the CGA text screen.
 *              It also supports a text cursor position stored in the hardware using ports.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf, 6.2.2024
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 30.6.2025
 */

use core::fmt::Write;
use crate::kernel::cpu::IoPort;
use spin::mutex::Mutex;

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
pub const CGA_ROWS: usize = 25;
pub const CGA_COLUMNS: usize = 80;

const CGA_BASE_ADDR: *mut u8 = 0xb8000 as *mut u8;

const CGA_INDEX_PORT: u16 = 0x3d4; // select register
const CGA_DATA_PORT: u16 = 0x3d5;  // read/write register
const CGA_HIGH_BYTE_CMD: u8 = 14;  // cursor high byte
const CGA_LOW_BYTE_CMD: u8 = 15;   // cursor high byte

pub struct CGA {
    index_port: IoPort,
    data_port: IoPort
}

impl Write for CGA {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.print_byte(byte),

                // not part of printable ASCII range
                _ => self.print_byte(0xfe),
            }
        }

        Ok(())
    }
}

impl CGA {
    /// Create a new CGA instance.
    const fn new() -> CGA {
        CGA {
            index_port: IoPort::new(CGA_INDEX_PORT),
            data_port: IoPort::new(CGA_DATA_PORT)
        }
    }

    /// Clear CGA screen and set cursor position to (0, 0).
    pub fn clear(&mut self) {
        // Unsafe because we are writing directly to memory using a pointer.
        // We ensure that the pointer is valid by using CGA_BASE_ADDR
        // and not writing beyond the screen size.
        unsafe {
            let screen_ptr = CGA_BASE_ADDR as *mut u16;
            for i in 0..(CGA_ROWS * CGA_COLUMNS) {
                // Write a space character and the standard attribute to the screen buffer.
                // If we set the attribute to 0, the cursor would not be visible.
                screen_ptr.offset(i as isize).write((b' ' as u16) | ((CGA_STD_ATTR as u16) << 8));
            }
        }
        
        self.setpos(0, 0);
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
        let high: u8;
        let low: u8;
        
        // Read the cursor position from the hardware.
        //
        // Unsafe because we are reading directly from hardware registers.
        // We ensure that the ports are valid by using CGA_INDEX_PORT and CGA_DATA_PORT in new()
        // and that no other code is accessing them at the same time
        // by using a mutable self reference.
        unsafe {
            self.index_port.outb(CGA_HIGH_BYTE_CMD);
            high = self.data_port.inb();
            self.index_port.outb(CGA_LOW_BYTE_CMD);
            low = self.data_port.inb();
        }

        // Calculate the cursor position.
        let pos = (high as usize) << 8 | (low as usize);
        let x = pos % CGA_COLUMNS;
        let y = pos / CGA_COLUMNS;

        (x, y)
    }

    /// Set cursor position `x`,`y` 
    pub fn setpos(&mut self, x: usize, y: usize) {
        if x > CGA_COLUMNS || y > CGA_ROWS {
            return;
        }

        // Calculate the cursor position.
        let pos = (y * CGA_COLUMNS + x) as u16;

        // Write the cursor position to the hardware.
        //
        // Unsafe because we are writing directly to hardware registers.
        // We ensure that the ports are valid by using CGA_INDEX_PORT and CGA_DATA_PORT in new()
        // and that no other code is accessing them at the same time
        // by using a mutable self reference. Furthermore, we check that the position is valid.
        unsafe {
            self.index_port.outb(CGA_HIGH_BYTE_CMD);
            self.data_port.outb((pos >> 8) as u8);
            self.index_port.outb(CGA_LOW_BYTE_CMD);
            self.data_port.outb(pos as u8);
        }
    }

    /// Print byte `b` at actual position cursor position `x`,`y`
    pub fn print_byte(&mut self, b: u8) {
        let pos = self.getpos();
        if b == b'\n' || b == b'\r' {
            // Handle new line character:
            // We set the position to the start of the next row.
            // If the cursor is already at the last row, we need to scroll.
            if pos.1 >= CGA_ROWS - 1 {
                self.scrollup();
                self.setpos(0, CGA_ROWS - 1);
            } else {
                self.setpos(0, pos.1 + 1);
            }
        } else {
            // Print the character
            self.show(pos.0, pos.1, b as char, CGA_STD_ATTR);

            // Update cursor position:
            // We move the cursor forward by one column.
            // If the cursor is already at the last column,
            // we set the position to the beginning of the next row.
            if pos.0 + 1 >= CGA_COLUMNS {
                self.setpos(0, pos.1 + 1)
            } else {
                self.setpos(pos.0 + 1, pos.1)
            };
        }
    }

    /// Scroll text lines by one to the top.
    pub fn scrollup(&mut self) {
        // Unsafe because we are writing directly to memory using a pointer.
        // We ensure that the pointer is valid by using CGA_BASE_ADDR
        // and not writing beyond the screen size.
        unsafe {
            // Copy the screen buffer upwards by one row.
            let scroll_ptr = CGA_BASE_ADDR.offset(2 * CGA_COLUMNS as isize);
            CGA_BASE_ADDR.copy_from(scroll_ptr, 2 * (CGA_ROWS - 1) * CGA_COLUMNS);
            
            // Clear the last row.
            let last_row = CGA_BASE_ADDR.offset(2 * ((CGA_ROWS - 1) * CGA_COLUMNS) as isize) as *mut u16;
            for i in 0..CGA_COLUMNS {
                // Write a space character and the standard attribute to the screen buffer.
                // If we set the attribute to 0, the cursor would not be visible.
                last_row.offset(i as isize).write((b' ' as u16) | ((CGA_STD_ATTR as u16) << 8));
            }
        }
    }

    /// Helper function returning an attribute byte for the given parameters `bg`, `fg`, and `blink`
    /// Note: Blinking characters do not work in QEMU, but work on real hardware.
    ///       Support for blinking characters is optional and can be removed, if you want.
    pub fn attribute(bg: Color, fg: Color, blink: bool) -> u8 {
        ((bg as u8) << 4) | (fg as u8) | (if blink { 0x80 } else { 0x00 })
    }
}