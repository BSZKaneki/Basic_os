use core::sync::atomic::{AtomicU64, Ordering};

// Keep this static strictly private inside this file.
// Nobody outside timer.rs can touch or modify this raw variable directly.
static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);

/// Get the current number of system ticks since boot.
/// Safe and accessible from anywhere (including main.rs).
pub fn get_ticks() -> u64 {
    TIMER_TICKS.load(Ordering::Relaxed)
}

/// Increment the system tick counter by 1.
/// Marked as `pub(crate)` so only modules inside our own kernel (like `idt.rs`)
/// can increment the clock, preventing main.rs from artificially altering time.
pub(crate) fn increment() {
    TIMER_TICKS.fetch_add(1, Ordering::Relaxed);
}