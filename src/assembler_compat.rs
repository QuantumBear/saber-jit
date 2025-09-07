// Temporary compatibility layer for the old assembler interface
// TODO: Replace with full dynasm-rs implementation

pub mod x64 {
    use std::ops::{Add, Mul};

    #[derive(Debug, Clone, Copy, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum R64 {
        rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp, r8, r9, r10, r11, r12, r13, r14, r15
    }

    impl Add<i32> for R64 {
        type Output = Addr;
        fn add(self, rhs: i32) -> Self::Output {
            Addr::BD(self, rhs)
        }
    }

    impl Mul<i32> for R64 {
        type Output = Addr;
        fn mul(self, rhs: i32) -> Self::Output {
            Addr::BD(self, rhs)
        }
    }

    impl Add<Addr> for R64 {
        type Output = Addr;
        fn add(self, _rhs: Addr) -> Self::Output {
            Addr::B(self)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Cond {
        E, L, G, NE, LE, GE
    }

    impl Cond {
        pub fn inverse(self) -> Self {
            match self {
                Cond::E => Cond::NE,
                Cond::NE => Cond::E,
                Cond::L => Cond::GE,
                Cond::GE => Cond::L,
                Cond::G => Cond::LE,
                Cond::LE => Cond::G,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Addr {
        B(R64),
        BD(R64, i32),
    }

    impl Add<i32> for Addr {
        type Output = Addr;
        fn add(self, rhs: i32) -> Self::Output {
            match self {
                Addr::B(reg) => Addr::BD(reg, rhs),
                Addr::BD(reg, offset) => Addr::BD(reg, offset + rhs),
            }
        }
    }

    // Re-export registers for old code
    pub use R64::*;

    // Arithmetic traits
    pub trait EmitPush<T> {
        fn push(&mut self, op: T) -> &mut Self;
    }

    pub trait EmitPop<T> {
        fn pop(&mut self, op: T) -> &mut Self;
    }

    pub trait EmitMov<D, S> {
        fn mov(&mut self, dst: D, src: S) -> &mut Self;
    }

    pub trait EmitArith<D, S> {
        fn add(&mut self, dst: D, src: S) -> &mut Self;
        fn sub(&mut self, dst: D, src: S) -> &mut Self;
        fn cmp(&mut self, dst: D, src: S) -> &mut Self;
    }

    pub trait EmitBranch<T> {
        fn call(&mut self, op: T) -> &mut Self;
        fn jmp(&mut self, op: T) -> &mut Self;
    }

    pub trait EmitLea<D, S> {
        fn lea(&mut self, dst: D, src: S) -> &mut Self;
    }

    pub trait EmitRet {
        fn ret(&mut self) -> &mut Self;
    }

    pub trait EmitNop {
        fn nop(&mut self) -> &mut Self;
    }
}

pub mod emit {
    use super::x64::*;

    #[derive(Debug)]
    pub struct Label(pub usize);

    impl Label {
        pub fn new() -> Self {
            Label(0)
        }

        pub fn from_offset(offset: usize) -> Self {
            Label(offset)
        }

        pub fn offset(&mut self) -> Option<usize> {
            Some(self.0)
        }
    }

    pub struct Emit {
        data: Vec<u8>,
        position: usize,
    }

    impl Emit {
        pub fn new() -> Self {
            Emit {
                data: Vec::new(),
                position: 0,
            }
        }

        pub fn as_ref(&self) -> &[u8] {
            &self.data
        }

        pub fn add(&mut self, _dst: R64, _offset: i32) -> &mut Self {
            // TODO: Implement with dynasm
            self
        }

        pub fn mov<D, S>(&mut self, _dst: D, _src: S) -> &mut Self {
            // TODO: Implement with dynasm
            self
        }

        pub fn call(&mut self, _target: &Addr) -> &mut Self {
            // TODO: Implement with dynasm
            self
        }

        pub fn call_r64(&mut self, _target: R64) -> &mut Self {
            // TODO: Implement with dynasm
            self
        }

        pub fn as_slice(&self) -> &[u8] {
            &self.data
        }

        pub fn here(&self) -> usize {
            self.position
        }

        pub fn create_label(&mut self) -> Label {
            Label(self.position)
        }

        pub fn bind_label(&mut self, _label: Label) {
            // TODO: Bind label to current position
        }

        pub fn bind(&mut self, _label: &mut Label) -> &mut Self {
            // TODO: Bind label to current position
            self
        }

        pub fn alloc<T>(&mut self, _info: T) {
            // TODO: Implement allocation
        }

        pub fn lea(&mut self, _dst: R64, _src: &mut Label) -> &mut Self {
            self
        }

        pub fn lea_addr(&mut self, _dst: R64, _src: &Addr) -> &mut Self {
            self
        }

        pub fn jcc(&mut self, _cond: Cond, _label: &mut Label) -> &mut Self {
            self
        }

        pub fn jne(&mut self, _label: &mut Label) -> &mut Self {
            self
        }

        pub fn je(&mut self, _label: &mut Label) -> &mut Self {
            self
        }

        pub fn jmp(&mut self, _label: &mut Label) -> &mut Self {
            self
        }

        pub fn jmp_addr(&mut self, _addr: &Addr) -> &mut Self {
            self
        }

        pub fn jle(&mut self, _label: &mut Label) -> &mut Self {
            self
        }

        pub fn sub(&mut self, _dst: R64, _offset: i32) -> &mut Self {
            self
        }

        pub fn add_addr(&mut self, _dst: R64, _src: &Addr) -> &mut Self {
            self
        }

        pub fn sub_addr(&mut self, _dst: R64, _src: &Addr) -> &mut Self {
            self
        }

        pub fn cmove(&mut self, _dst: R64, _src: R64) -> &mut Self {
            self
        }

        pub fn cmovcc(&mut self, _cond: Cond, _dst: R64, _src: R64) -> &mut Self {
            self
        }
    }

    // Implementation stubs for all the traits
    impl EmitPush<R64> for Emit {
        fn push(&mut self, _op: R64) -> &mut Self { self }
    }

    impl EmitPush<&Addr> for Emit {
        fn push(&mut self, _op: &Addr) -> &mut Self { self }
    }

    impl EmitPop<R64> for Emit {
        fn pop(&mut self, _op: R64) -> &mut Self { self }
    }

    impl EmitMov<R64, R64> for Emit {
        fn mov(&mut self, _dst: R64, _src: R64) -> &mut Self { self }
    }

    impl EmitMov<R64, i64> for Emit {
        fn mov(&mut self, _dst: R64, _src: i64) -> &mut Self { self }
    }

    impl EmitMov<&Addr, R64> for Emit {
        fn mov(&mut self, _dst: &Addr, _src: R64) -> &mut Self { self }
    }

    impl EmitMov<R64, &Addr> for Emit {
        fn mov(&mut self, _dst: R64, _src: &Addr) -> &mut Self { self }
    }

    impl EmitArith<R64, R64> for Emit {
        fn add(&mut self, _dst: R64, _src: R64) -> &mut Self { self }
        fn sub(&mut self, _dst: R64, _src: R64) -> &mut Self { self }
        fn cmp(&mut self, _dst: R64, _src: R64) -> &mut Self { self }
    }

    impl EmitArith<R64, &Addr> for Emit {
        fn add(&mut self, _dst: R64, _src: &Addr) -> &mut Self { self }
        fn sub(&mut self, _dst: R64, _src: &Addr) -> &mut Self { self }
        fn cmp(&mut self, _dst: R64, _src: &Addr) -> &mut Self { self }
    }

    impl EmitArith<R64, i32> for Emit {
        fn add(&mut self, _dst: R64, _src: i32) -> &mut Self { self }
        fn sub(&mut self, _dst: R64, _src: i32) -> &mut Self { self }
        fn cmp(&mut self, _dst: R64, _src: i32) -> &mut Self { self }
    }

    impl EmitArith<R64, i64> for Emit {
        fn add(&mut self, _dst: R64, _src: i64) -> &mut Self { self }
        fn sub(&mut self, _dst: R64, _src: i64) -> &mut Self { self }
        fn cmp(&mut self, _dst: R64, _src: i64) -> &mut Self { self }
    }

    impl EmitBranch<R64> for Emit {
        fn call(&mut self, _op: R64) -> &mut Self { self }
        fn jmp(&mut self, _op: R64) -> &mut Self { self }
    }

    impl EmitBranch<&Addr> for Emit {
        fn call(&mut self, _op: &Addr) -> &mut Self { self }
        fn jmp(&mut self, _op: &Addr) -> &mut Self { self }
    }

    impl EmitBranch<&mut Label> for Emit {
        fn call(&mut self, _op: &mut Label) -> &mut Self { self }
        fn jmp(&mut self, _op: &mut Label) -> &mut Self { self }
    }

    impl EmitLea<R64, &mut Label> for Emit {
        fn lea(&mut self, _dst: R64, _src: &mut Label) -> &mut Self { self }
    }

    impl EmitLea<R64, &Addr> for Emit {
        fn lea(&mut self, _dst: R64, _src: &Addr) -> &mut Self { self }
    }

    impl EmitRet for Emit {
        fn ret(&mut self) -> &mut Self { self }
    }

    impl EmitNop for Emit {
        fn nop(&mut self) -> &mut Self { self }
    }
}

pub mod mem {
    use dynasmrt::{ExecutableBuffer, DynasmApi};
    use std::sync::OnceLock;

    pub struct JitMem {
        buffer: ExecutableBuffer,
    }

    impl JitMem {
        pub fn new(data: &[u8]) -> Result<Self, std::io::Error> {
            // Simple stub - create empty executable buffer for now
            let mut buffer = dynasmrt::x64::Assembler::new()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create assembler"))?;
            
            // Add some placeholder code
            dynasm::dynasm!(buffer; nop);
            
            let exec_buffer = buffer.finalize()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to finalize assembler"))?;
            
            Ok(JitMem { buffer: exec_buffer })
        }

        pub fn start(&self) -> *const u8 {
            self.buffer.ptr(dynasmrt::AssemblyOffset(0))
        }

        pub fn as_mut(&mut self) -> Result<&mut [u8], std::io::Error> {
            // Return a proper error
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Mutable access not supported"))
        }
    }
}

// Stub functions
pub fn emit_nop_until_aligned(_emit: &mut emit::Emit, _alignment: usize) {
    // TODO: Implement proper alignment
}