use cef_ui_bindings_linux_x86_64::{cef_log_items_t, cef_log_severity_t};

/// Log severity levels.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogSeverity {
    /// Default logging (currently info).
    Default,

    /// Verbose logging.
    Verbose,

    /// Info logging.
    Info,

    /// Warning logging.
    Warning,

    /// Error logging.
    Error,

    /// Fatal logging.
    Fatal,

    /// Disable logging to file for all messages, and to
    /// stderr for messages with severity less than fatal.
    Disable
}

impl Default for LogSeverity {
    fn default() -> Self {
        LogSeverity::Default
    }
}

impl From<cef_log_severity_t> for LogSeverity {
    fn from(value: cef_log_severity_t) -> Self {
        match value {
            cef_log_severity_t::LOGSEVERITY_DEFAULT => Self::Default,
            cef_log_severity_t::LOGSEVERITY_VERBOSE => Self::Verbose,
            cef_log_severity_t::LOGSEVERITY_INFO => Self::Info,
            cef_log_severity_t::LOGSEVERITY_WARNING => Self::Warning,
            cef_log_severity_t::LOGSEVERITY_ERROR => Self::Error,
            cef_log_severity_t::LOGSEVERITY_FATAL => Self::Fatal,
            cef_log_severity_t::LOGSEVERITY_DISABLE => Self::Disable
        }
    }
}

impl From<LogSeverity> for cef_log_severity_t {
    fn from(value: LogSeverity) -> Self {
        match value {
            LogSeverity::Default => Self::LOGSEVERITY_DEFAULT,
            LogSeverity::Verbose => Self::LOGSEVERITY_VERBOSE,
            LogSeverity::Info => Self::LOGSEVERITY_INFO,
            LogSeverity::Warning => Self::LOGSEVERITY_WARNING,
            LogSeverity::Error => Self::LOGSEVERITY_ERROR,
            LogSeverity::Fatal => Self::LOGSEVERITY_FATAL,
            LogSeverity::Disable => Self::LOGSEVERITY_DISABLE
        }
    }
}

/// Log items prepended to each log line.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogItems {
    /// Prepend the default list of items.
    Default,

    /// Prepend no items.
    None,

    /// Prepend the process ID.
    FlagProcessId,

    /// Prepend the thread ID.
    FlagThreadId,

    /// Prepend the timestamp.
    FlagTimeStamp,

    /// Prepend the tick count.
    FlagTickCount
}

impl Default for LogItems {
    fn default() -> Self {
        LogItems::Default
    }
}

impl From<cef_log_items_t> for LogItems {
    fn from(value: cef_log_items_t) -> Self {
        match value {
            cef_log_items_t::LOG_ITEMS_DEFAULT => Self::Default,
            cef_log_items_t::LOG_ITEMS_NONE => Self::None,
            cef_log_items_t::LOG_ITEMS_FLAG_PROCESS_ID => Self::FlagProcessId,
            cef_log_items_t::LOG_ITEMS_FLAG_THREAD_ID => Self::FlagThreadId,
            cef_log_items_t::LOG_ITEMS_FLAG_TIME_STAMP => Self::FlagTimeStamp,
            cef_log_items_t::LOG_ITEMS_FLAG_TICK_COUNT => Self::FlagTickCount
        }
    }
}

impl From<LogItems> for cef_log_items_t {
    fn from(value: LogItems) -> Self {
        match value {
            LogItems::Default => Self::LOG_ITEMS_DEFAULT,
            LogItems::None => Self::LOG_ITEMS_NONE,
            LogItems::FlagProcessId => Self::LOG_ITEMS_FLAG_PROCESS_ID,
            LogItems::FlagThreadId => Self::LOG_ITEMS_FLAG_THREAD_ID,
            LogItems::FlagTimeStamp => Self::LOG_ITEMS_FLAG_TIME_STAMP,
            LogItems::FlagTickCount => Self::LOG_ITEMS_FLAG_TICK_COUNT
        }
    }
}
