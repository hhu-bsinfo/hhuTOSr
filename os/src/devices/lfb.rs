use alloc::string::ToString;
use spin::Once;
use crate::devices::font_8x8;
use crate::library::mutex::Mutex;

/// Global Linear Framebuffer (LFB) instance.
static LFB: Once<Mutex<LFB>> = Once::new();

/// Initialize the Linear Framebuffer (LFB).
pub fn init_lfb(addr: *mut u8, pitch: u32, width: u32, height: u32, bpp: u8) {
    LFB.call_once(|| { 
        Mutex::new(LFB::new(addr, pitch, width, height, bpp))
    });
}

/// Global access to the Linear Framebuffer (LFB).
pub fn get_lfb() -> &'static Mutex<LFB> {
    LFB.get().expect("LFB not initialized")
}

/// Represents a Linear Framebuffer (LFB) for graphics output.
/// The framebuffer is expected to be in 32-bit ARGB format.
pub struct LFB {
    addr: *mut u8,
    pitch: u32,
    width: u32,
    height: u32,
}

unsafe impl Send for LFB {}
unsafe impl Sync for LFB {}

/// Converts RGB values to a 32-bit color value in ARGB format.
pub const fn color(red: u8, green: u8, blue: u8) -> u32 {
    ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)
}

// ANSI colors
pub const BLACK: u32 = color(0, 0, 0);
pub const RED: u32 = color(170, 0, 0);
pub const GREEN: u32 = color(0, 170, 0);
pub const YELLOW: u32 = color(170, 170, 0);
pub const BROWN: u32 = color(170, 85, 0);
pub const BLUE: u32 = color(0, 0, 170);
pub const MAGENTA: u32 = color(170, 0, 170);
pub const CYAN: u32 = color(0, 170, 170);
pub const WHITE: u32 = color(170, 170, 170);

// HHU primary colors
pub const HHU_BLUE: u32 = color(0, 106, 179);
pub const HHU_BLUE_70: u32 = color(54, 128, 179);
pub const HHU_BLUE_50: u32 = color(90, 142, 179);
pub const HHU_BLUE_30: u32 = color(125, 157, 179);
pub const HHU_BLUE_10: u32 = color(161, 172, 179);

pub const HHU_GRAY: u32 = color(217, 218, 219);
pub const HHU_LIGHT_GRAY: u32 = color(236, 237, 237);

// HHU secondary colors
pub const HHU_GREEN: u32 = color(151, 191, 13);
pub const HHU_GREEN_70: u32 = color(159, 191, 57);
pub const HHU_GREEN_50: u32 = color(168, 191, 96);
pub const HHU_GREEN_30: u32 = color(177, 191, 134);
pub const HHU_GREEN_10: u32 = color(186, 191, 172);

pub const HHU_RED: u32 = color(190, 10, 38);
pub const HHU_RED_70: u32 = color(190, 57, 78);
pub const HHU_RED_50: u32 = color(190, 95, 110);
pub const HHU_RED_30: u32 = color(190, 133, 142);
pub const HHU_RED_10: u32 = color(190, 171, 174);

pub const HHU_DARK_BLUE: u32 = color(0, 56, 101);
pub const HHU_DARK_BLUE_70: u32 = color(30, 70, 101);
pub const HHU_DARK_BLUE_50: u32 = color(51, 79, 101);
pub const HHU_DARK_BLUE_30: u32 = color(71, 88, 101);
pub const HHU_DARK_BLUE_10: u32 = color(91, 97, 101);

pub const HHU_YELLOW: u32 = color(242, 148, 0);
pub const HHU_YELLOW_70: u32 = color(242, 176, 73);
pub const HHU_YELLOW_50: u32 = color(242, 195, 121);
pub const HHU_YELLOW_30: u32 = color(242, 214, 169);
pub const HHU_YELLOW_10: u32 = color(242, 233, 218);

pub const HHU_TURQUOISE: u32 = color(50, 184, 201);
pub const HHU_TURQUOISE_70: u32 = color(60, 185, 201);
pub const HHU_TURQUOISE_50: u32 = color(101, 190, 201);
pub const HHU_TURQUOISE_30: u32 = color(141, 194, 201);
pub const HHU_TURQUOISE_10: u32 = color(181, 199, 201);

impl LFB {
    /// Create a new Linear Framebuffer (LFB) instance.
    const fn new(addr: *mut u8, pitch: u32, width: u32, height: u32, bpp: u8) -> LFB {
        if bpp != 32 {
            panic!("Only 32-bit per pixel (ARGB) format is supported for LFB");
        }
        
        LFB { addr, pitch, width, height }
    }

    /// Get the resolution of the framebuffer as (width, height).
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    
    /// Get the width and height of a character in the font used by the framebuffer.
    pub fn get_char_dimensions(&self) -> (u32, u32) {
        (font_8x8::CHAR_WIDTH, font_8x8::CHAR_HEIGHT)
    }
    
    /// Clear the framebuffer by filling it with black pixels.
    pub fn clear(&mut self) {
        unsafe {
            self.addr.write_bytes(0, (self.pitch * self.height) as usize);
        }
    }
    
    /// Draw a pixel at the specified (x, y) coordinates with the given color.
    /// This method checks the bounds of the framebuffer before drawing
    /// and omits drawing if the coordinates are out of bounds.
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            unsafe {
                self.draw_pixel_unchecked(x, y, color);
            }
        }
    }
    
    /// Draw a pixel at the specified (x, y) coordinates with the given color.
    /// This method does not check the bounds of the framebuffer.
    /// This is faster than `draw_pixel` but the caller must ensure that the coordinates are valid.
    /// Drawing outside the framebuffer may lead to undefined behavior.
    pub unsafe fn draw_pixel_unchecked(&mut self, x: u32, y: u32, color: u32) {
        let offset = (y * self.pitch + x * 4) as usize;
        
        unsafe {
            let pixel_ptr = self.addr.add(offset) as *mut u32;
            *pixel_ptr = color;
        }
    }
    
    /// Draw a bitmap image at the specified (x, y) coordinates with the given width and height.
    pub fn draw_bitmap(&mut self, x: u32, y: u32, width: u32, height: u32, bitmap: &[u8]) {
        let draw_height = if y + height > self.height {
            self.height - y
        } else {
            height
        };
        
        let draw_width = if x + width > self.width {
            self.width - x
        } else {
            width
        };

        let xpos: u32 = x;
        let ypos: u32 = y;

        for y in 0..draw_height {
            for x in 0..draw_width {
                let index = ((y * width + x) * 3) as usize;
                let red = bitmap[index];
                let green = bitmap[index + 1];
                let blue = bitmap[index + 2];

                unsafe {
                    self.draw_pixel_unchecked(xpos + x, ypos + y, color(red, green, blue));
                }
            }
        }
    }

    /// Get the pixel data for a character from the font data.
    fn get_char_pixels(c: char) -> &'static [u8] {
        let char_mem_size = (font_8x8::CHAR_WIDTH + (8 >> 1)) / 8 * font_8x8::CHAR_HEIGHT;
        let start = (char_mem_size * c as u32) as usize;
        let end = start + char_mem_size as usize;

        &font_8x8::DATA[start..end]
    }

    /// Draw a single character at the specified (x, y) coordinates with the given color.
    pub fn draw_char(&mut self, x: u32, y: u32, color: u32, c: char) {
        self.draw_str(x, y, color, str::from_utf8(&[c as u8]).unwrap());
    }

    /// Draw a string at the specified (x, y) coordinates with the given color.
    pub fn draw_str(&mut self, x: u32, y: u32, color: u32, str: &str) {
        let char_width  = font_8x8::CHAR_WIDTH;
        let char_height = font_8x8::CHAR_HEIGHT;
        let width_byte = if char_width % 8 == 0 {
            char_width / 8
        } else {
            char_width / 8 + 1
        };

        let mut current_char_xpos = x;

        for c in str.chars() {
            let char_pixels = LFB::get_char_pixels(c);
            let mut pixel_index = 0;
            
            for y_offset in 0..char_height {
                let mut xpos = current_char_xpos;
                let ypos = y + y_offset;

                for byte in 0..width_byte {
                    for bit in (0..8).rev() {
                        if ((1 << bit) & char_pixels[pixel_index]) != 0 {
                            self.draw_pixel(xpos, ypos, color);
                        }

                        xpos += 1;
                    }
                }

                pixel_index += 1;
            }

            current_char_xpos += char_width;
        }
    }
}