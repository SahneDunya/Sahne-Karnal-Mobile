#![no_std]
use crate::drivers::pci;

// AHCI MMIO adresleri (PCI üzerinden bulunur)
// ...

pub fn init() {
    // PCI üzerinden AHCI denetleyicisini bul
    // ...

    // AHCI denetleyicisini başlat
    // ...
}

pub fn read_sectors(lba: u64, count: u32, buffer: *mut u8) {
    // AHCI komutlarını kullanarak sektörleri oku
    // ...
}

pub fn write_sectors(lba: u64, count: u32, buffer: *const u8) {
    // AHCI komutlarını kullanarak sektörleri yaz
    // ...
}