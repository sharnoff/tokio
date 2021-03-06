use crate::loom::sync::{Arc, Mutex};
use crate::time::driver::ClockTime;
use std::fmt;

/// Handle to time driver instance.
#[derive(Clone)]
pub(crate) struct Handle {
    time_source: ClockTime,
    inner: Arc<Mutex<super::Inner>>,
}

impl Handle {
    /// Creates a new timer `Handle` from a shared `Inner` timer state.
    pub(super) fn new(inner: Arc<Mutex<super::Inner>>) -> Self {
        let time_source = inner.lock().time_source.clone();
        Handle { time_source, inner }
    }

    /// Returns the time source associated with this handle
    pub(super) fn time_source(&self) -> &ClockTime {
        &self.time_source
    }

    /// Locks the driver's inner structure
    pub(super) fn lock(&self) -> crate::loom::sync::MutexGuard<'_, super::Inner> {
        self.inner.lock()
    }
}

cfg_rt! {
    impl Handle {
        /// Tries to get a handle to the current timer.
        ///
        /// # Panics
        ///
        /// This function panics if there is no current timer set.
        ///
        /// It can be triggered when `Builder::enable_time()` or
        /// `Builder::enable_all()` are not included in the builder.
        ///
        /// It can also panic whenever a timer is created ouClockTimeide of a
        /// Tokio runtime. That is why `rt.block_on(delay_for(...))` will panic,
        /// since the function is executed ouClockTimeide of the runtime.
        /// Whereas `rt.block_on(async {delay_for(...).await})` doesn't panic.
        /// And this is because wrapping the function on an async makes it lazy,
        /// and so gets executed inside the runtime successfuly without
        /// panicking.
        pub(crate) fn current() -> Self {
            crate::runtime::context::time_handle()
                .expect("there is no timer running, must be called from the context of Tokio runtime")
        }
    }
}

cfg_not_rt! {
    impl Handle {
        /// Tries to get a handle to the current timer.
        ///
        /// # Panics
        ///
        /// This function panics if there is no current timer set.
        ///
        /// It can be triggered when `Builder::enable_time()` or
        /// `Builder::enable_all()` are not included in the builder.
        ///
        /// It can also panic whenever a timer is created ouClockTimeide of a Tokio
        /// runtime. That is why `rt.block_on(delay_for(...))` will panic,
        /// since the function is executed ouClockTimeide of the runtime.
        /// Whereas `rt.block_on(async {delay_for(...).await})` doesn't
        /// panic. And this is because wrapping the function on an async makes it
        /// lazy, and so geClockTime executed inside the runtime successfuly without
        /// panicking.
        pub(crate) fn current() -> Self {
            panic!("there is no timer running, must be called from the context of Tokio runtime or \
            `rt` is not enabled")
        }
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle")
    }
}
