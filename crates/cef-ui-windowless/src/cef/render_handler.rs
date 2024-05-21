use cef_ui::{
    AccessibilityHandler, Browser, DragData, DragOperations, HorizontalAlignment, PaintElementType,
    Point, Range, Rect, RenderHandlerCallbacks, ScreenInfo, Size, TextInputMode, TouchHandleState
};
use std::{
    ffi::c_void,
    sync::{Arc, Mutex, RwLock}
};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::EventLoopProxy
};

use crate::messages::MessageLoopEvent;

/// The render handler state.
#[derive(Debug, Clone, Default)]
pub struct RenderState {
    /// The window's physical size.
    pub physical_size: PhysicalSize<u32>,

    /// The window's logical size.
    pub logical_size: LogicalSize<u32>,

    /// The window's scale factor.
    pub scale_factor: f32,

    /// The rendered frame data.
    pub frame_data: Vec<u8>,

    /// The size of the rendered frame.
    pub frame_size: PhysicalSize<u32>
}

/// CEF render handler callbacks.
pub struct MyRenderHandlerCallbacks {
    /// The event loop proxy. This is how we send
    /// events to the main thread's event loop.
    pub proxy: Arc<Mutex<EventLoopProxy<MessageLoopEvent>>>,

    /// The render state.
    pub render_state: Arc<RwLock<RenderState>>
}

#[allow(unused_variables)]
impl RenderHandlerCallbacks for MyRenderHandlerCallbacks {
    fn get_accessibility_handler(&mut self) -> Option<AccessibilityHandler> {
        None
    }

    fn get_root_screen_rect(&mut self, browser: Browser) -> Option<Rect> {
        None
    }

    fn get_view_rect(&mut self, browser: Browser) -> Rect {
        let render_state = self.render_state.read().unwrap();

        Rect {
            x:      0,
            y:      0,
            width:  render_state.logical_size.width as i32,
            height: render_state.logical_size.height as i32
        }
    }

    fn get_screen_point(&mut self, browser: Browser, view: &Point) -> Option<Point> {
        None
    }

    fn get_screen_info(&mut self, browser: Browser) -> Option<ScreenInfo> {
        let rect = self.get_view_rect(browser);
        let render_state = self.render_state.read().unwrap();
        let scale_factor = render_state.scale_factor;

        Some(ScreenInfo {
            device_scale_factor: scale_factor,
            depth: 32,
            depth_per_component: 8,
            is_monochrome: false,
            rect,
            available_rect: rect
        })
    }

    fn on_popup_show(&mut self, browser: Browser, show: bool) {}

    fn on_popup_size(&mut self, browser: Browser, rect: &Rect) {}

    fn on_paint(
        &mut self,
        browser: Browser,
        paint_element_type: PaintElementType,
        dirty_rects: &[Rect],
        buffer: &[u8],
        width: usize,
        height: usize
    ) {
        let mut render_state = self.render_state.write().unwrap();

        // Update the frame data.
        let frame_data = &mut render_state.frame_data;
        frame_data.resize_with(buffer.len(), Default::default);
        frame_data.copy_from_slice(buffer);

        // Update the frame size.
        render_state.frame_size = PhysicalSize::new(width as u32, height as u32);

        // Notify the main thread.
        self.proxy
            .lock()
            .unwrap()
            .send_event(MessageLoopEvent::NewCefFrameReady)
            .ok();
    }

    fn on_accelerated_paint(
        &mut self,
        browser: Browser,
        paint_element_type: PaintElementType,
        dirty_rects: &[Rect],
        shared_handle: *mut c_void
    ) {
    }

    fn get_touch_handle_size(
        &mut self,
        _browser: Browser,
        _orientation: HorizontalAlignment
    ) -> Size {
        Size {
            width:  16,
            height: 16
        }
    }

    fn on_touch_handle_state_changed(&mut self, browser: Browser, state: &TouchHandleState) {}

    fn start_dragging(
        &mut self,
        browser: Browser,
        drag_data: DragData,
        allowed_ops: DragOperations,
        drag_start: &Point
    ) -> bool {
        false
    }

    fn update_drag_cursor(&mut self, browser: Browser, operation: DragOperations) {}

    fn on_scroll_offset_changed(&mut self, browser: Browser, x: f64, y: f64) {}

    fn on_ime_composition_range_changed(
        &mut self,
        browser: Browser,
        selected_range: &Range,
        character_bounds: &[Rect]
    ) {
    }

    fn on_text_selection_changed(
        &mut self,
        browser: Browser,
        selected_text: &str,
        selected_range: &Range
    ) {
    }

    fn on_virtual_keyboard_requested(&mut self, browser: Browser, input_mode: TextInputMode) {}
}
