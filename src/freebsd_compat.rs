use core::marker::PhantomData;

#[cfg(target_arch = "aarch64")]
const JMP_BUF_SIZE: usize = 32 * 4;
#[cfg(target_arch = "x86_64")]
const JMP_BUF_SIZE: usize = 12 * 2;
#[cfg(target_arch = "riscv64")]
const JMP_BUF_SIZE: usize = 64 * 2;

/// `JmpBufFields` are the accessible fields when viewed via a JmpBuf pointer.
/// But also: You shouldn't be poking at these!
#[repr(C)]
pub struct JmpBufFields {
    _buf: [u32; JMP_BUF_SIZE],
    _neither_send_nor_sync: PhantomData<*const u8>,
}

/// `SigJmpBufFields` are the accessible fields when viewed via a SigJmpBuf pointer.
/// But also: You shouldn't be poking at these!
#[repr(C)]
pub struct SigJmpBufFields {
    _buf: [u32; JMP_BUF_SIZE],
    _neither_send_nor_sync: PhantomData<*const u8>,
}

pub type JmpBufStruct = JmpBufFields;
pub type SigJmpBufStruct = SigJmpBufFields;
