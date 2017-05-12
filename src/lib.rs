#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {  }
}

#[no_mangle]
pub extern fn kmain() -> ! {
    kprintln("Hello World!");
    kprintln_with_colors("Hello World!", KColor::Black, KColor::Yellow);

    loop {  }
}

const VGA: u64 = 0xb8000;

pub enum KColor {
 Black = 0x0,
 Blue = 0x1,
 Green = 0x2,
 Cyan = 0x3,
 Red = 0x4,
 Magenta = 0x5,
 Brown = 0x6,
 Gray = 0x7,
 DarkGray = 0x8,
 BrightBlue = 0x9,
 BrightGreen = 0xA,
 BrightCyan = 0xB,
 BrightRed = 0xC,
 BrightMagenta = 0xD,
 Yellow = 0xE,
 White = 0xF
}

#[no_mangle]
pub extern fn kprintln(msg: &str) {
    kprintln_with_colors(msg, KColor::Black, KColor::Green);
}

#[no_mangle]
pub fn kprintln_with_colors(msg: &str, bg_color: KColor, fg_color: KColor) {
    let bg_color = bg_color as u16 * 0x1000;
    let fg_color = fg_color as u16 * 0x100;

    unsafe {
        let mut vga = VGA as *mut u64;

        let mut i = 0;
        let bytes = msg.as_bytes();
        while i < bytes.len() {
            vga = (VGA + (i as u64) * 2) as *mut u64;
            *vga = (bg_color + fg_color + bytes[i] as u16) as u64;
            i += 1;
        }
    };
}
