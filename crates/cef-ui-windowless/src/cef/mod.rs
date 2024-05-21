use crate::messages::MessageLoopEvent;

use self::{
    app::MyAppCallbacks,
    browser_process_handler::MyBrowserProcessHandlerCallbacks,
    client::MyClientCallbacks,
    render_handler::{MyRenderHandlerCallbacks, RenderState}
};
use anyhow::Result;
use cef_ui::{
    App, Browser, BrowserHost, BrowserProcessHandler, BrowserSettings, Client, Color, Context,
    LogSeverity, MainArgs, RenderHandler, Settings, WindowInfo
};
use std::{
    fs::create_dir_all,
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
    time::Instant
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopProxy,
    window::Window
};

mod app;
mod browser_process_handler;
mod client;
mod render_handler;

/// The settings for CEF.
pub struct CefSettings {
    /// The log severity.
    pub log_severity: LogSeverity,

    /// The event loop proxy.
    pub proxy: Arc<Mutex<EventLoopProxy<MessageLoopEvent>>>,

    /// Frame rate.
    pub frame_rate: u32
}

/// Browser-specific state.
struct BrowserState {
    /// The browser.
    pub browser: Browser,

    /// The render state.
    pub render_state: Arc<RwLock<RenderState>>
}

/// The wrapper around CEF.
pub struct Cef {
    // The settings.
    settings: CefSettings,

    // The CEF context.
    context: Context,

    /// Instants in the future when we should schedule
    /// work again. This is ultimately driven by CEF.
    scheduled_updates: Vec<Instant>,

    /// The browser state.
    browser_state: Option<BrowserState>
}

impl Cef {
    pub fn new(settings: CefSettings) -> Result<Self> {
        // The command line arguments.
        let main_args = MainArgs::new()?;

        // Ensure the root cache directory exists.
        let root_cache_dir = Self::get_root_cache_dir()?;

        // Prepare the outermost CEF settings. We will drive the
        // event loop ourselves and use offscreen rendering.
        let cef_settings = Settings::new()
            .log_severity(settings.log_severity)
            .root_cache_path(&root_cache_dir)?
            .external_message_pump(true)
            .windowless_rendering_enabled(true)
            .no_sandbox(false);

        // Setup the browser process handler. This is how
        // we manually drive CEF's message pump loop.
        let browser_process_handler =
            BrowserProcessHandler::new(MyBrowserProcessHandlerCallbacks {
                proxy: settings.proxy.clone()
            });

        // Create the outermost CEF application.
        let app = App::new(MyAppCallbacks {
            browser_process_handler
        });

        // Create the CEF context which is the outermost way we interact
        // with CEF, mainly for booting it up and shutting it down.
        let context = Context::new(main_args, cef_settings, Some(app));

        Ok(Self {
            settings,
            context,
            scheduled_updates: Vec::new(),
            browser_state: None
        })
    }

    /// If this is a CEF subprocess, emit the error
    /// code that the subprocess should return.
    pub fn is_cef_subprocess(&self) -> Option<i32> {
        self.context.is_cef_subprocess()
    }

    /// Initialize CEF.
    pub fn initialize(&mut self, window: Arc<Window>) -> Result<()> {
        self.context.initialize()?;
        self.create_browser(window, "https://www.google.com")?;

        Ok(())
    }

    /// Shutdown CEF.
    pub fn shutdown(&mut self) {
        self.context.shutdown();
    }

    /// Create a browser of a specific type.
    fn create_browser(&mut self, window: Arc<Window>, url: &str) -> Result<()> {
        // Create the window info, making sure it's windowless.
        let window_info = WindowInfo::new().windowless_rendering_enabled(true);

        // Create the browser settings.
        let browser_settings = BrowserSettings::new()
            .windowless_frame_rate(self.settings.frame_rate as i32)
            .background_color(&Color {
                a: 0,
                r: 255,
                g: 255,
                b: 255
            });

        // The render settings. This is how we propagate
        // window size changes to the render handler.
        let render_state = Arc::new(RwLock::new(RenderState {
            physical_size: window.inner_size(),
            logical_size:  window
                .inner_size()
                .to_logical::<u32>(window.scale_factor()),
            scale_factor:  window.scale_factor() as f32,
            frame_data:    Vec::new(),
            frame_size:    window.inner_size()
        }));

        // Create the render handler. This is how we extract CEF's frame
        // buffer and send it to the main thread to be rendered.
        let render_handler = RenderHandler::new(MyRenderHandlerCallbacks {
            proxy:        self.settings.proxy.clone(),
            render_state: render_state.clone()
        });

        // The browser-specific client.
        let client = Client::new(MyClientCallbacks { render_handler });

        // The actual interface to the browser.
        let browser = BrowserHost::create_browser_sync(
            &window_info,
            client,
            url,
            &browser_settings,
            None,
            None
        );

        // Store the browser state.
        self.browser_state = Some(BrowserState {
            browser,
            render_state
        });

        Ok(())
    }

    /// Call this every time you receive an event.
    pub fn update(&mut self, event: &Event<MessageLoopEvent>, window: Arc<Window>) -> Result<()> {
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Notify the browser that a resize occurred.
                if let Some(browser_state) = &self.browser_state {
                    let scale_factor = window.scale_factor();

                    // Update the render state.
                    {
                        let mut render_state = browser_state
                            .render_state
                            .write()
                            .unwrap();

                        render_state.physical_size = *size;
                        render_state.logical_size = (*size).to_logical::<u32>(scale_factor);
                        render_state.scale_factor = scale_factor as f32;
                    }

                    // Notify the host that a resize occurred.
                    browser_state
                        .browser
                        .get_host()?
                        .was_resized()?;

                    // I don't understand why the size change doesn't eventually
                    // trigger an update on the main thread, so do it manually.
                    self.context.do_message_loop_work();
                }
            },

            Event::AboutToWait => {
                let mut future_updates = Vec::new();
                let mut ready = false;

                // If any scheduled update was scheduled prior to the
                // current instant, then manually step the event loop.
                for instant in self.scheduled_updates.iter() {
                    if *instant > Instant::now() {
                        future_updates.push(*instant);
                    } else {
                        ready = true;
                    }
                }

                if ready {
                    self.context.do_message_loop_work();
                }

                self.scheduled_updates = future_updates;
            },

            Event::UserEvent(event) => match event {
                // If it's time to schedule CEF work, do so.
                MessageLoopEvent::ScheduleCefWork(instant) => {
                    self.scheduled_updates
                        .push(*instant);
                },

                // If a new frame is ready to be rendered, create a new
                // texture from the raw data and notify the UI about it.
                MessageLoopEvent::NewCefFrameReady => {
                    if let Some(browser_state) = &self.browser_state {
                        let _render_state = browser_state
                            .render_state
                            .read()
                            .unwrap();

                        // // Create a new texture for the CEF frame.
                        // let texture = game_window.set_cef_texture(
                        //     *browser_type,
                        //     &render_state.frame_data,
                        //     &render_state.frame_size
                        // );

                        // // Tell the UI about the CEF texture.
                        // ui.set_cef_texture(
                        //     *browser_type,
                        //     texture,
                        //     &render_state.frame_size,
                        //     render_state.scale_factor
                        // );
                    }
                }
            },

            _ => {}
        }

        Ok(())
    }

    // TODO: Make this platform-specific!

    /// Ensure the root cache directory exists.
    fn get_root_cache_dir() -> Result<PathBuf> {
        let path = PathBuf::from("/tmp/cef-ui-windowless");
        if !path.exists() {
            create_dir_all(&path)?;
        }

        Ok(path)
    }
}
