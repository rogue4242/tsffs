extern "C" {
    /// Discard recorded future events and forget them
    pub fn CORE_discard_future();
}

/// Discard future events that are scheduled
pub fn discard_future() {
    unsafe { CORE_discard_future() };
}
