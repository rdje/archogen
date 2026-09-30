//! The `riscv-virt-up` architecture spike (leaf `M2.8.4`).
//!
//! `M2.8` asks for startup, a timer interrupt, interrupt return, observable output and context preservation, run in
//! the emulator. This image does each once, in machine mode, and reports it on the UART `/chosen`'s `stdout-path`
//! names. It then stops the machine through the test device behind `/poweroff`, so QEMU's exit status is the verdict:
//! `0` when every step held.
//!
//! Every address is one `docs/targets/riscv-virt-up.dtb.summary.md` records: RAM at `0x8000_0000`, the `ns16550a`
//! at `0x1000_0000`, the `sifive,clint0` at `0x0200_0000`, and the test device at `0x0010_0000`.
//!
//! ⚠️ A measurement of the target, not the runtime. Nothing here is on the generation path, and nothing in `rt-core`
//! depends on it.

#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, Ordering};

/// The UART's transmit holding register, and its line status register.
const UART_THR: *mut u8 = 0x1000_0000 as *mut u8;
const UART_LSR: *const u8 = 0x1000_0005 as *const u8;
/// The CLINT's compare register for hart 0, and its time counter (the SiFive CLINT layout).
const MTIMECMP: *mut u64 = 0x0200_4000 as *mut u64;
const MTIME: *const u64 = 0x0200_BFF8 as *const u64;
/// The test device: `0x5555` powers off with status 0; `(code << 16) | 0x3333` with status `code`.
const FINISHER: *mut u32 = 0x0010_0000 as *mut u32;
/// `mcause` for a machine-timer interrupt: the interrupt bit and cause 7.
const MACHINE_TIMER: usize = (1 << 63) | 7;
/// One millisecond at the 10 MHz timebase `/cpus` records.
const ONE_MS: u64 = 10_000;

/// Set by the trap handler when the machine-timer interrupt has been taken.
static FIRED: AtomicBool = AtomicBool::new(false);

global_asm!(
    r#"
    .section .text.init, "ax"
    .globl _start
_start:
    la sp, __stack_top
    la t0, __bss_start
    la t1, __bss_end
1:  bgeu t0, t1, 2f
    sd zero, 0(t0)
    addi t0, t0, 8
    j 1b
2:  la t0, trap_entry
    csrw mtvec, t0
    call spike_main
3:  wfi
    j 3b

    .text
    .balign 4
    .globl trap_entry
trap_entry:
    addi sp, sp, -256
    sd x1, 0(sp)
    sd x5, 8(sp)
    sd x6, 16(sp)
    sd x7, 24(sp)
    sd x8, 32(sp)
    sd x9, 40(sp)
    sd x10, 48(sp)
    sd x11, 56(sp)
    sd x12, 64(sp)
    sd x13, 72(sp)
    sd x14, 80(sp)
    sd x15, 88(sp)
    sd x16, 96(sp)
    sd x17, 104(sp)
    sd x18, 112(sp)
    sd x19, 120(sp)
    sd x20, 128(sp)
    sd x21, 136(sp)
    sd x22, 144(sp)
    sd x23, 152(sp)
    sd x24, 160(sp)
    sd x25, 168(sp)
    sd x26, 176(sp)
    sd x27, 184(sp)
    sd x28, 192(sp)
    sd x29, 200(sp)
    sd x30, 208(sp)
    sd x31, 216(sp)
    csrr a0, mcause
    call spike_trap
    ld x1, 0(sp)
    ld x5, 8(sp)
    ld x6, 16(sp)
    ld x7, 24(sp)
    ld x8, 32(sp)
    ld x9, 40(sp)
    ld x10, 48(sp)
    ld x11, 56(sp)
    ld x12, 64(sp)
    ld x13, 72(sp)
    ld x14, 80(sp)
    ld x15, 88(sp)
    ld x16, 96(sp)
    ld x17, 104(sp)
    ld x18, 112(sp)
    ld x19, 120(sp)
    ld x20, 128(sp)
    ld x21, 136(sp)
    ld x22, 144(sp)
    ld x23, 152(sp)
    ld x24, 160(sp)
    ld x25, 168(sp)
    ld x26, 176(sp)
    ld x27, 184(sp)
    ld x28, 192(sp)
    ld x29, 200(sp)
    ld x30, 208(sp)
    ld x31, 216(sp)
    addi sp, sp, 256
    j trap_exit
"#
);

// The return from a trap. The `clobber` build is the spike's own negative control: it corrupts one pattern register
// on the way out, and `scripts/target_spike.sh` requires that build to report `context CLOBBERED`. So the context
// check is shown able to fail, on the emulated hardware, every time it runs.
#[cfg(not(feature = "clobber"))]
global_asm!(
    ".text",
    ".balign 4",
    ".globl trap_exit",
    "trap_exit:",
    "mret"
);
#[cfg(feature = "clobber")]
global_asm!(
    ".text",
    ".balign 4",
    ".globl trap_exit",
    "trap_exit:",
    "li x6, 0",
    "mret"
);

fn put(text: &str) {
    for byte in text.bytes() {
        // SAFETY: the `ns16550a` the device tree names; bit 5 of its line status is "transmit holding empty".
        unsafe {
            while read_volatile(UART_LSR) & 0x20 == 0 {}
            write_volatile(UART_THR, byte);
        }
    }
}

fn finish(code: u32) -> ! {
    let value = if code == 0 {
        0x5555
    } else {
        (code << 16) | 0x3333
    };
    // SAFETY: the test device the device tree's `/poweroff` and `/reboot` use.
    unsafe { write_volatile(FINISHER, value) };
    loop {
        // SAFETY: waiting for the power-off to take effect.
        unsafe { asm!("wfi") };
    }
}

/// The trap handler's Rust half. Every register the interrupted code could hold has been saved by `trap_entry`.
#[no_mangle]
extern "C" fn spike_trap(mcause: usize) {
    if mcause == MACHINE_TIMER {
        // SAFETY: the CLINT's compare register; the largest value disarms the interrupt.
        unsafe { write_volatile(MTIMECMP, u64::MAX) };
        FIRED.store(true, Ordering::SeqCst);
        put("spike: machine-timer interrupt taken\n");
    } else {
        put("spike: an unexpected trap\n");
        finish(3);
    }
}

/// A value per register the wait loop holds, so a register the handler failed to restore is caught.
const PATTERN: [usize; 23] = [
    0x5a5a_0001,
    0x5a5a_0002,
    0x5a5a_0003,
    0x5a5a_0004,
    0x5a5a_0005,
    0x5a5a_0006,
    0x5a5a_0007,
    0x5a5a_0008,
    0x5a5a_0009,
    0x5a5a_000a,
    0x5a5a_000b,
    0x5a5a_000c,
    0x5a5a_000d,
    0x5a5a_000e,
    0x5a5a_000f,
    0x5a5a_0010,
    0x5a5a_0011,
    0x5a5a_0012,
    0x5a5a_0013,
    0x5a5a_0014,
    0x5a5a_0015,
    0x5a5a_0016,
    0x5a5a_0017,
];

#[no_mangle]
extern "C" fn spike_main() -> ! {
    put("spike: boot on riscv-virt-up\n");

    // Arm the CLINT one millisecond ahead, then enable the machine-timer interrupt.
    // SAFETY: the CLINT the device tree names, and the machine-mode CSRs that gate its interrupt.
    unsafe {
        write_volatile(MTIMECMP, read_volatile(MTIME) + ONE_MS);
        asm!("csrs mie, {}", in(reg) 1usize << 7);
        asm!("csrs mstatus, {}", in(reg) 1usize << 3);
    }
    put("spike: timer armed\n");

    // Hold a known value in 23 registers, the caller-saved and the callee-saved alike, and wait for the interrupt.
    // An interrupt is asynchronous, so the handler must restore every one of them.
    let p = PATTERN;
    let flag = core::ptr::addr_of!(FIRED) as usize;
    let mut r: [usize; 23] = p;
    // SAFETY: the loop reads `FIRED` through its address and leaves every pattern register to the interrupt.
    unsafe {
        asm!(
            "2:",
            "wfi",
            "lbu t0, 0(a0)",
            "beqz t0, 2b",
            in("a0") flag,
            out("t0") _,
            inout("t1") r[0], inout("t2") r[1], inout("t3") r[2], inout("t4") r[3], inout("t5") r[4],
            inout("t6") r[5], inout("a1") r[6], inout("a2") r[7], inout("a3") r[8], inout("a4") r[9],
            inout("a5") r[10], inout("a6") r[11], inout("a7") r[12], inout("s2") r[13], inout("s3") r[14],
            inout("s4") r[15], inout("s5") r[16], inout("s6") r[17], inout("s7") r[18], inout("s8") r[19],
            inout("s9") r[20], inout("s10") r[21], inout("s11") r[22],
        );
    }
    put("spike: interrupt returned\n");

    if r == p {
        put("spike: context preserved in 23 registers\n");
        put("spike: ok\n");
        finish(0);
    }
    put("spike: context CLOBBERED\n");
    finish(2)
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    put("spike: panic\n");
    finish(4)
}
