//! Collection of known leak techniques, and also failed techniques.

use super::*;

/// Sanity check that simply dropping implicitly does not leak.
#[test]
fn implicit_drop() -> Result {
    expect_no_leak(|_leak_me| {})
}

/// The mem::forget technique, the most straight-forward way to leak
#[test]
fn mem_forget() -> Result {
    expect_leak(|leak_me| std::mem::forget(leak_me))
}

/// The Box::leak technique, another straight-forward way to leak
#[test]
fn box_leak() -> Result {
    expect_leak(|leak_me| {
        let _ = Box::leak(Box::new(leak_me));
    })
}

/// The Rc-cycle technique
#[test]
fn rc_cycle() -> Result {
    use std::cell::RefCell;
    use std::rc::Rc;

    struct Cycle<T>(T, RefCell<Option<Rc<Cycle<T>>>>);
    expect_leak(|leak_me| {
        let cycle = Rc::new(Cycle(leak_me, RefCell::new(None)));
        let cycle_clone = Rc::clone(&cycle);
        *cycle.1.borrow_mut() = Some(cycle_clone);
    })
}

/// The Arc-cycle technique, the thread safe version of the Rc-technique
#[test]
fn arc_cycle() -> Result {
    use std::sync::{Arc, Mutex};

    struct Cycle<T>(T, Mutex<Option<Arc<Cycle<T>>>>);
    expect_leak(|leak_me| {
        let cycle = Arc::new(Cycle(leak_me, Mutex::new(None)));
        let cycle_clone = Arc::clone(&cycle);
        *cycle.1.lock().expect("could not acquire mutex") = Some(cycle_clone);
    })
}

/// Attempt to leak by panicking in drop in an array, before the LeakMe data is
/// dropped. This is currently not possible due to Rust mitigating this for
/// arrays and slices. Note that arrays drop their elements in order.
#[test]
fn panic_drop_in_array() -> Result {
    expect_no_leak(|leak_me| {
        assert!(std::panic::catch_unwind(|| {
            let _: [Box<dyn Drop>; 2] = [Box::new(PanicDropper), Box::new(leak_me)];
        })
        .is_err());
    })
}


/// Attempt to leak by panicking in drop in a Vec. This is currently not
/// possible, due to mitigations by Vec (using slices).
#[test]
fn panic_drop_in_vec() -> Result {
    expect_no_leak(|leak_me| {
        assert!(std::panic::catch_unwind(|| {
            let _: Vec::<Box<dyn Drop>> = vec![Box::new(PanicDropper), Box::new(leak_me)];
        })
        .is_err());
    })
}
