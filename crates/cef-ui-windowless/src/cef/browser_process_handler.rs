use cef_ui::{
    BrowserProcessHandlerCallbacks, Client, CommandLine, PreferenceRegistrar, PreferencesType
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant}
};
use winit::event_loop::EventLoopProxy;

use crate::messages::MessageLoopEvent;

/// CEF browser process handler callbacks.
pub struct MyBrowserProcessHandlerCallbacks {
    /// The event loop proxy. This is how we send
    /// events to the main thread's event loop.
    pub proxy: Arc<Mutex<EventLoopProxy<MessageLoopEvent>>>
}

#[allow(unused_variables)]
impl BrowserProcessHandlerCallbacks for MyBrowserProcessHandlerCallbacks {
    fn on_register_custom_preferences(
        &mut self,
        _preferences_type: PreferencesType,
        _registrar: &mut PreferenceRegistrar
    ) {
    }

    fn on_context_initialized(&mut self) {}

    fn on_before_child_process_launch(&mut self, _command_line: CommandLine) {}

    fn on_already_running_app_relaunch(
        &mut self,
        _command_line: CommandLine,
        _current_directory: &str
    ) -> bool {
        false
    }

    fn on_schedule_message_pump_work(&mut self, delay_ms: i64) {
        let delay = Instant::now() + Duration::from_millis(delay_ms as u64);

        self.proxy
            .lock()
            .unwrap()
            .send_event(MessageLoopEvent::ScheduleCefWork(delay))
            .ok();
    }

    fn get_default_client(&mut self) -> Option<Client> {
        None
    }
}
