mod naive_copying;
pub use self::naive_copying::{GcState, FullGcArgs};

pub const INFO_FRESH_TAG: usize = 0;
pub const INFO_MARKED_TAG: usize = 1;
