use cef_ui::{
    ClientCallbacks, ContextMenuHandler, KeyboardHandler, LifeSpanHandler, RenderHandler
};

/// CEF client callbacks.
pub struct MyClientCallbacks {
    /// The render handler. This is how we capture the frame
    /// buffer and pass it along to the game window.
    pub render_handler: RenderHandler
}

impl ClientCallbacks for MyClientCallbacks {
    fn get_context_menu_handler(&mut self) -> Option<ContextMenuHandler> {
        None
    }

    fn get_keyboard_handler(&mut self) -> Option<KeyboardHandler> {
        None
    }

    fn get_life_span_handler(&mut self) -> Option<LifeSpanHandler> {
        None
    }

    fn get_render_handler(&mut self) -> Option<RenderHandler> {
        Some(self.render_handler.clone())
    }
}
