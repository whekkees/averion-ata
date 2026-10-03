use core::arch::asm;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inb(port: u16) -> u8 {
    let val: u8;
    asm!("in al, dx", in("dx") port, out("al") val, options(nomem, nostack, preserves_flags));
    val
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inw(port: u16,) -> u16{
    let val: u16;
    asm!("in al, dx", in("dx") port, out("ax") val, options(nomem, nostack, preserves_flags));
    val
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn outw(port: u16, val: u16) {
    asm!("out dx, al", in("dx") port, in("ax") val, options(nomem, nostack, preserves_flags));
}
