// extern crate assembler;  // Removed - using dynasm-rs directly
extern crate byteorder;
#[macro_use]
extern crate log;
extern crate fnv;
extern crate bit_vec;

pub mod assembler_compat;  // Temporary compatibility layer
pub mod ast;
pub mod rt;
pub mod interp;
pub mod c0;
//pub mod c1;
