use core::arch::asm;

use crate::MAX_BACKTRACE_ADRESSES;

/// Registers saved in trap handler
#[doc(hidden)]
#[allow(missing_docs)]
#[derive(Default, Clone, Copy)]
#[repr(C)]
pub(crate) struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub gp: usize,
    pub tp: usize,
    pub sp: usize,
    pub pc: usize,
    pub mstatus: usize,
    pub mcause: usize,
    pub mtval: usize,
}

impl core::fmt::Debug for TrapFrame {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            fmt,
            "TrapFrame
PC=0x{:08x}         RA/x1=0x{:08x}      SP/x2=0x{:08x}      GP/x3=0x{:08x}      TP/x4=0x{:08x}
T0/x5=0x{:08x}      T1/x6=0x{:08x}      T2/x7=0x{:08x}      S0/FP/x8=0x{:08x}   S1/x9=0x{:08x}
A0/x10=0x{:08x}     A1/x11=0x{:08x}     A2/x12=0x{:08x}     A3/x13=0x{:08x}     A4/x14=0x{:08x}
A5/x15=0x{:08x}     A6/x16=0x{:08x}     A7/x17=0x{:08x}     S2/x18=0x{:08x}     S3/x19=0x{:08x}
S4/x20=0x{:08x}     S5/x21=0x{:08x}     S6/x22=0x{:08x}     S7/x23=0x{:08x}     S8/x24=0x{:08x}
S9/x25=0x{:08x}     S10/x26=0x{:08x}    S11/x27=0x{:08x}    T3/x28=0x{:08x}     T4/x29=0x{:08x}
T5/x30=0x{:08x}     T6/x31=0x{:08x}

MSTATUS=0x{:08x}
MCAUSE=0x{:08x}
MTVAL=0x{:08x}
        ",
            self.pc,
            self.ra,
            self.gp,
            self.sp,
            self.tp,
            self.t0,
            self.t1,
            self.t2,
            self.s0,
            self.s1,
            self.a0,
            self.a1,
            self.a2,
            self.a3,
            self.a4,
            self.a5,
            self.a6,
            self.a7,
            self.s2,
            self.s3,
            self.s4,
            self.s5,
            self.s6,
            self.s7,
            self.s8,
            self.s9,
            self.s10,
            self.s11,
            self.t3,
            self.t4,
            self.t5,
            self.t6,
            self.mstatus,
            self.mcause,
            self.mtval,
        )
    }
}

/// Get an array of backtrace addresses.
///
/// This needs `force-frame-pointers` enabled.
pub fn backtrace() -> impl Iterator<Item = usize> {
    let fp = unsafe {
        let mut _tmp: usize;
        asm!("mv {0}, x8", out(reg) _tmp); // aka s0
        _tmp
    };

    FrameIter { fp }
        .skip(0 /* TODO */)
        .take(MAX_BACKTRACE_ADRESSES)
}

pub struct FrameIter {
    pub fp: usize,
}

impl Iterator for FrameIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if !crate::is_valid_ram_address(self.fp) {
            return None;
        }

        let last = self.fp;
        let ra = unsafe {
            let ra = (self.fp as *const usize).offset(-1).read(); // RA/PC
            self.fp = (self.fp as *const usize).offset(-2).read(); // next FP
            ra
        };

        if ra == 0 || self.fp == last {
            return None;
        }

        Some(ra)
    }
}
