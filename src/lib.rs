use core::mem::ManuallyDrop;

/// Defer execution of a closure until the return value is dropped.
pub fn defer<F>(f: F) -> impl Drop
where
    F: FnOnce(),
{
    struct Defer<F: FnOnce()>(ManuallyDrop<F>);

    impl<F: FnOnce()> Drop for Defer<F> {
        fn drop(&mut self) {
            let f: F = unsafe { ManuallyDrop::take(&mut self.0) };
            f();
        }
    }

    Defer(ManuallyDrop::new(f))
}

/// Defer execution of a closure until the current scope end.
#[macro_export]
macro_rules! defer {
    ($e:expr) => {
        let _defer = $crate::defer(|| $e);
    };
}

#[test]
fn test() {
    use std::cell::RefCell;

    let i = RefCell::new(0);

    {
        let _d = defer(|| *i.borrow_mut() += 1);
        assert_eq!(*i.borrow(), 0);
    }

    assert_eq!(*i.borrow(), 1);
}

#[test]
fn test_macro() {
    use std::cell::RefCell;

    let i = RefCell::new(0);

    {
        defer!(*i.borrow_mut() += 1);
        assert_eq!(*i.borrow(), 0);
    }

    assert_eq!(*i.borrow(), 1);
}
