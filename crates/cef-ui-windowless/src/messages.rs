use std::time::Instant;

/// This is necessary to send asynchroneous messages
/// from other threads to the winit event loop.
#[derive(Debug, Clone)]
pub enum MessageLoopEvent {
    /// Sent when CEF work should be scheduled. Here, the
    /// nested value is the time in the future when the
    /// CEF work should be scheduled.
    ScheduleCefWork(Instant),

    /// Handle this when a new CEF frame is
    /// ready for the specific browser type.
    NewCefFrameReady
}
