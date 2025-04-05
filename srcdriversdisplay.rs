#![no_std]
use core::ptr::write_volatile;

pub trait FramebufferInfo {
    fn address(&self) -> usize;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pitch(&self) -> usize;
    fn init(&self); // Yeni: Başlatma fonksiyonu
}

pub struct Framebuffer<T: FramebufferInfo> {
    info: T,
}

impl<T: FramebufferInfo> Framebuffer<T> {
    pub const fn new(info: T) -> Self {
        Self { info }
    }

    pub fn init(&self){
        self.info.init();
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.info.width() && y < self.info.height() {
            let offset = y * self.info.pitch() + x * 4;
            unsafe {
                write_volatile((self.info.address() + offset) as *mut u32, color);
            }
        }
    }

    pub fn clear(&mut self, color: u32) {
        for y in 0..self.info.height() {
            for x in 0..self.info.width() {
                self.write_pixel(x, y, color);
            }
        }
    }
}

// platform/my_riscv_board.rs
pub struct MyRiscvFramebufferInfo;

impl FramebufferInfo for MyRiscvFramebufferInfo {
    fn address(&self) -> usize {
        0x40000000
    }
    fn width(&self) -> usize {
        800
    }
    fn height(&self) -> usize {
        600
    }
    fn pitch(&self) -> usize {
        800 * 4
    }

    fn init(&self) {
        // Donanıma özgü başlatma işlemleri buraya
    }
}

// main.rs
use platform::my_riscv_board::MyRiscvFramebufferInfo;

static mut FRAMEBUFFER: Framebuffer<MyRiscvFramebufferInfo> = Framebuffer::new(MyRiscvFramebufferInfo);

fn main() {
    unsafe {
        FRAMEBUFFER.init();
        FRAMEBUFFER.clear(0xFFFFFFFF); // Beyaz
    }
    loop {}
}