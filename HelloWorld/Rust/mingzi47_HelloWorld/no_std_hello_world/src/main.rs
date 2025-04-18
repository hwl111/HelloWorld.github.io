#![no_std]
#![no_main]

const UART0: *mut u8 = 0x1000_0000 as *mut u8;
const EXIT_ADDR: u64 = 0x100000;
const EXIT_CODE: u64 = 0x5555;

static HELLO: &[u8] = b"\x1b[31mHello World!\x1b[0m\n";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    HELLO
        .iter()
        .for_each(|&c| {
            unsafe {
                // output
                UART0.write_volatile(c);
            }
        });

    // exit qemu
    unsafe {
        core::arch::asm!(
            "sw {0}, 0({1})",
            in(reg)EXIT_CODE, in(reg)EXIT_ADDR,
        )
    }

    loop {
        unsafe {
            core::arch::asm!(
                "wfi",
                options(nomem, nostack)
            );
        }
    }
}

use core::panic::PanicInfo;

// no std, must impl 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!(
                "wfi",
                options(nomem, nostack)
            );
        }
    }
}
