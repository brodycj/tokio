use core::cell::UnsafeCell;
use core::fmt;
use core::ops;

/// `AtomicUsize` providing an additional `unsync_load` function.
pub(crate) struct AtomicUsize {
    inner: UnsafeCell<core::sync::atomic::AtomicUsize>,
}

unsafe impl Send for AtomicUsize {}
unsafe impl Sync for AtomicUsize {}

impl AtomicUsize {
    pub(crate) const fn new(val: usize) -> AtomicUsize {
        let inner = UnsafeCell::new(core::sync::atomic::AtomicUsize::new(val));
        AtomicUsize { inner }
    }

    /// Performs an unsynchronized load.
    ///
    /// # Safety
    ///
    /// All mutations must have happened before the unsynchronized load.
    /// Additionally, there must be no concurrent mutations.
    pub(crate) unsafe fn unsync_load(&self) -> usize {
        // See <https://github.com/tokio-rs/tokio/issues/6155>
        self.load(core::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn with_mut<R>(&mut self, f: impl FnOnce(&mut usize) -> R) -> R {
        // safety: we have mutable access
        f(unsafe { (*self.inner.get()).get_mut() })
    }
}

impl ops::Deref for AtomicUsize {
    type Target = core::sync::atomic::AtomicUsize;

    fn deref(&self) -> &Self::Target {
        // safety: it is always safe to access `&self` fns on the inner value as
        // we never perform unsafe mutations.
        unsafe { &*self.inner.get() }
    }
}

impl ops::DerefMut for AtomicUsize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // safety: we hold `&mut self`
        unsafe { &mut *self.inner.get() }
    }
}

impl fmt::Debug for AtomicUsize {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(fmt)
    }
}
