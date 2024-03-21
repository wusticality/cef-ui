mod accessibility_handler;
mod app;
mod browser;
mod browser_process_handler;
mod callbacks;
mod client;
mod color;
mod command_line;
mod context;
mod context_menu_handler;
mod drag;
mod events;
mod extension;
mod extension_handler;
mod frame;
mod ime;
mod keyboard_handler;
mod life_span_handler;
mod macros;
mod main_args;
mod navigation_entry;
mod platform;
mod process;
mod refcounted;
mod render_handler;
mod request;
mod request_context;
mod request_context_handler;
mod request_handler;
mod resource_request_handler;
mod response;
mod settings;
mod shared_memory_region;
mod ssl;
mod string;
mod time;
mod types;
mod url_request;
mod values;
mod x509_certificate;

pub use accessibility_handler::*;
pub use app::*;
pub use browser::*;
pub use browser_process_handler::*;
pub use callbacks::*;
pub use client::*;
pub use color::*;
pub use command_line::*;
pub use context::*;
pub use context_menu_handler::*;
pub use drag::*;
pub use events::*;
pub use extension::*;
pub use extension_handler::*;
pub use frame::*;
pub use ime::*;
pub use life_span_handler::*;
pub use macros::*;
pub use main_args::*;
pub use navigation_entry::*;
pub use platform::*;
pub use process::*;
pub use refcounted::*;
pub use render_handler::*;
pub use request::*;
pub use request_context::*;
pub use request_context_handler::*;
pub use request_handler::*;
pub use resource_request_handler::*;
pub use response::*;
pub use settings::*;
pub use shared_memory_region::*;
pub use ssl::*;
pub use string::*;
pub use time::*;
pub use types::*;
pub use url_request::*;
pub use values::*;
pub use x509_certificate::*;

// TODO: Evaluate that your code is actually save! You were doing some unsafe
//  things getting pointers to CefString's and then letting the original value
//  be dropped before you used the pointer. This is because you were using the
//  original value by value instead of by reference.

// TODO: Maybe don't provide default implementations for callbacks?
//  If anything, check all your default implementations to make sure
//  they return sane defaults per the documentation.

// TODO: Return Option<T> for all getters on types. Otherwise you may
//  be returning invalid default values.

// TODO: Make a macro for all the pointer conversion functions.

// TODO: Fix issues where we're using command ids.

// TODO: Always return Option<String>.
