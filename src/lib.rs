//! This crate challenges you to leak non-`'static` data past its
//! lifetime in safe Rust.
//!
//! See README.md

#[cfg(test)]
pub mod tests;

use std::sync::atomic::{AtomicBool, Ordering};

/// The type that you should attempt to leak.
pub struct LeakMe<'a> {
    alive: &'a AtomicBool,
}

/// Result type for this crate
pub type Result = std::result::Result<(), Error>;

/// Runs `f` with a `LeakMe` instance. Returns true if the
/// function successfully leaked the instance, and false otherwise.
pub fn check<F: FnOnce(LeakMe)>(f: F) -> bool {
    let alive = AtomicBool::new(true);
    {
        let leak_me = LeakMe { alive: &alive };
        f(leak_me);
    }
    alive.load(Ordering::SeqCst)
}

/// Returns an error if `f` **does not** leak the `LeakMe` instance.
pub fn expect_leak<F: FnOnce(LeakMe)>(f: F) -> Result {
    if check(f) {
        Ok(())
    } else {
        Err(Error::NotLeaked)
    }
}

/// Returns an error if `f` leaks the `LeakMe` instance.
pub fn expect_no_leak<F: FnOnce(LeakMe)>(f: F) -> Result {
    if check(f) {
        Err(Error::Leaked)
    } else {
        Ok(())
    }
}

impl<'a> Drop for LeakMe<'a> {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
    }
}

/// Utility type which panics in its drop implementation. Useful in combination
/// with some techniques.
pub struct PanicDropper;

impl Drop for PanicDropper {
    fn drop(&mut self) {
        panic!("Panicked in drop");
    }
}

/// Error type for this crate
#[derive(Debug)]
pub enum Error {
    NotLeaked,
    Leaked,
}
