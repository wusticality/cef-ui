use cef_ui::{AppCallbacks, BrowserProcessHandler, CommandLine};

/// CEF application callbacks.
pub(crate) struct MyAppCallbacks {
    /// The browser process handler. This is ultimately
    /// what drives the updating of CEF's internals.
    pub browser_process_handler: BrowserProcessHandler
}

#[allow(unused_variables)]
impl AppCallbacks for MyAppCallbacks {
    fn on_before_command_line_processing(
        &mut self,
        process_type: Option<&str>,
        command_line: Option<CommandLine>
    ) {
        // If this is the browser process, try and enable the mock keychain.
        // Otherwise, launching the app will prompt the user for their password.
        #[cfg(target_os = "macos")]
        if process_type.is_none() {
            if let Some(command_line) = command_line {
                command_line
                    .append_switch("--use-mock-keychain")
                    .unwrap();
            }
        }
    }

    fn get_browser_process_handler(&mut self) -> Option<BrowserProcessHandler> {
        Some(self.browser_process_handler.clone())
    }
}
